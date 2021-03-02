use byteorder::{LittleEndian, WriteBytesExt};
use failure::{Error, ResultExt};
use log::{debug, info};
use std::io::{self};
use std::path::{Path, PathBuf};

use crate::address_space::AddressSpace;
use crate::bump_ptr_alloc::{BumpPointerAlloc, ChainedAlloc};
use crate::constants::PAGE_SIZE;
use crate::elf::Elf;
use crate::elf_writer;
use crate::interval::Interval;
use crate::page_table;
use crate::phys_mem::PhysMemory;
use crate::runtypes;

impl From<&runtypes::Configuration> for PhysMemory {
    fn from(system: &runtypes::Configuration) -> Self {
        PhysMemory::new(
            system
                .available_memory
                .iter()
                .map(|fm| Interval::new_with_size(fm.start, fm.size))
                .map(|i| BumpPointerAlloc::new(i, PAGE_SIZE))
                .collect::<ChainedAlloc<_>>(),
        )
    }
}

fn to_kernel_as(
    process: &runtypes::Process,
    user_binaries: &Path,
) -> Result<(Elf, AddressSpace), Error> {
    let kernel_path: PathBuf = [user_binaries, Path::new(&process.binary)].iter().collect();
    info!("Using {} as kernel binary", kernel_path.display(),);

    let kernel_elf = Elf::new(&kernel_path).context("Failed to load kernel ELF")?;

    let mut kernel_as = AddressSpace::from(&kernel_elf);
    kernel_as.extend(
        process
            .resources
            .iter()
            .filter_map(|(_, r)| r.opt_region.clone())
            .map(|vr| (&vr).into()),
    );

    Ok((kernel_elf, kernel_as))
}

fn to_user_as(
    process: &runtypes::Process,
    user_binaries: &Path,
    kernel_as: &AddressSpace,
) -> Result<AddressSpace, Error> {
    let user_path: PathBuf = [user_binaries, Path::new(&process.binary)].iter().collect();
    info!(
        "Using {} as binary for process {}",
        user_path.display(),
        process.name
    );

    let mut user_as = AddressSpace::from(&Elf::new(&user_path).context("Failed to load user ELF")?);

    user_as.extend(process.anon_mem.iter().map(|vr| vr.into()));

    user_as.extend(
        process
            .resources
            .iter()
            .filter_map(|(_, r)| r.opt_region.clone())
            .map(|vr| (&vr).into()),
    );

    // Make mappings available at the user privilege.
    //
    // BEWARE: This should not be called after merging in the kernel mappings to avoid making kernel
    // mappings available to user code as well.
    user_as.make_user();

    user_as.merge_from(kernel_as);

    debug!(
        "User address space for process {} is: {:#?}",
        process.name, user_as
    );

    Ok(user_as)
}

/// Convert a vector of 64-bit integers pointers to byte data.
fn u64_to_byte_data(pts: &[u64]) -> Result<Vec<u8>, Error> {
    let mut data = vec![];

    for &satp in pts {
        data.write_u64::<LittleEndian>(satp)?;
    }

    Ok(data)
}

/// Return the physical address of a symbol.
fn sym_paddr(name: &str, elf: &Elf, addr_space: &AddressSpace) -> Result<u64, Error> {
    let vaddr = elf
        .symbols
        .get(name)
        .cloned()
        .ok_or_else(|| format_err!("Failed to look up virtual address of symbol '{}'", name))?;

    addr_space.lookup_phys(vaddr).ok_or_else(|| {
        format_err!(
            "Failed to look up physical address of symbol '{}' (vaddr {:#x})",
            name,
            vaddr
        )
    })
}

fn process_entry(user_root: &Path, process: &runtypes::Process) -> Result<u64, Error> {
    let binary_path: PathBuf = [user_root, Path::new(&process.binary)].iter().collect();
    let elf = Elf::new(&binary_path).context("Failed to load process ELF")?;

    Ok(elf.entry)
}

pub fn generate(system: &runtypes::Configuration, user_binaries: &Path) -> Result<(), Error> {
    let (kernel_elf, mut kernel_as) = to_kernel_as(&system.kernel, user_binaries)?;
    let mut pmem: PhysMemory = system.into();

    debug!("Kernel address space is: {:#?}", kernel_as);

    // We allocate backing store for the kernel once, so we do not re-allocate it for every user
    // address space.
    kernel_as.fixate(&mut pmem)?;
    debug!("Kernel address space fixated to: {:#?}", kernel_as);

    let user_ass = system
        .processes
        .iter()
        .map(|(_, p)| -> Result<AddressSpace, Error> {
            to_user_as(p, user_binaries, &kernel_as)?.fixated(&mut pmem)
        })
        .collect::<Result<Vec<AddressSpace>, Error>>()?;

    info!("Generating page tables");

    let user_satps = user_ass
        .iter()
        .map(|a| page_table::generate(page_table::Format::RiscvSv32, a, &mut pmem))
        .collect::<Result<Vec<u64>, Error>>()?;

    let user_pcs = system
        .processes
        .iter()
        .map(|(_, p)| -> Result<u64, Error> { process_entry(user_binaries, p) })
        .collect::<Result<Vec<u64>, Error>>()?;

    info!("Patching kernel binary");

    // Patch the page table pointer the kernel boots with.
    pmem.write(
        sym_paddr("BOOT_SATP", &kernel_elf, &kernel_as).context("Failed to patch kernel SATP")?,
        &u64_to_byte_data(&user_satps[0..1])?,
    );

    // Patch the page tables of each user process.
    pmem.write(
        sym_paddr("USER_SATPS", &kernel_elf, &kernel_as)
            .context("Failed to patch user process SATPs")?,
        &u64_to_byte_data(&user_satps)?,
    );

    // Patch thread entry points.
    pmem.write(
        sym_paddr("USER_PCS", &kernel_elf, &kernel_as)
            .context("Failed to patch user process entry points")?,
        &u64_to_byte_data(&user_pcs)?,
    );

    info!("Boot image needs {} KiB of RAM.", pmem.size() >> 10);

    if atty::is(atty::Stream::Stdout) {
        Err(format_err!(
            "Refusing to write binary data to a terminal. Please redirect output to a stream."
        ))
    } else {
        let stdout = io::stdout();
        let out_buf = &mut stdout.lock();

        info!("Serializing boot image");

        elf_writer::write(
            out_buf,
            elf_writer::Format::Elf32,
            kernel_as
                .lookup_phys(kernel_elf.entry)
                .ok_or_else(|| format_err!("Failed to resolve vaddr {:#x}", kernel_elf.entry))?,
            &pmem,
        )?;

        info!("Finished writing boot image");
        Ok(())
    }
}
