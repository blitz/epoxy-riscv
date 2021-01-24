use failure::{Error, ResultExt};
use log::{debug, info};
use std::path::{Path, PathBuf};

use crate::address_space::AddressSpace;
use crate::bump_ptr_alloc::{BumpPointerAlloc, ChainedAlloc};
use crate::constants::PAGE_SIZE;
use crate::elf::Elf;
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

    user_as.add((&process.stack).into());
    user_as.extend(process.resources.iter().map(|(_, r)| r.into()));

    debug!(
        "User address space for process {} is: {:#?}",
        process.name, user_as
    );

    user_as.merge_from(kernel_as);
    Ok(user_as)
}

pub fn generate(
    system: &runtypes::Configuration,
    kernel_binary: &Path,
    user_binaries: &Path,
) -> Result<(), Error> {
    info!("Using {} as kernel", kernel_binary.display());

    let kernel_elf = Elf::new(kernel_binary).context("Failed to load kernel ELF")?;
    let mut kernel_as = AddressSpace::from(&kernel_elf);
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

    let user_satps = user_ass
        .iter()
        .map(|a| page_table::generate(page_table::Format::RiscvSv32, a, &mut pmem))
        .collect::<Result<Vec<u64>, Error>>()?;

    let pt_sym = "USER_SATPS";
    let pt_vaddr = kernel_elf
        .symbols
        .get(pt_sym)
        .cloned()
        .ok_or_else(|| format_err!("Failed to find location to patch page table pointers"))?;
    let pt_paddr = kernel_as
        .lookup_phys(pt_vaddr)
        .ok_or_else(|| format_err!("Failed to resolve vaddr {}", pt_vaddr))?;
    debug!(
        "Page tables need to be patched at vaddr {:#x} paddr {:#x}",
        pt_vaddr, pt_paddr
    );

    // TODO Use byteorder crate to serialize SATPs into [u8].

    todo!("generate page tables and patch them into pmem");
    todo!("generate ELF boot image")
}
