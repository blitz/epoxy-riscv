use elfy::Elf;
use failure::{Error, ResultExt};
use log::{debug, info};
use std::path::{Path, PathBuf};

use crate::address_space::AddressSpace;
use crate::bump_ptr_alloc::{BumpPointerAlloc, ChainedAlloc};
use crate::constants::PAGE_SIZE;
use crate::elf;
use crate::interval::Interval;
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

    let mut user_as = AddressSpace::from(&Elf::load(user_path).context("Failed to load user ELF")?);

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

    let kernel_elf = Elf::load(kernel_binary).context("Failed to load kernel ELF")?;
    let mut kernel_as = AddressSpace::from(&kernel_elf);
    let mut pmem: PhysMemory = system.into();

    debug!("Kernel address space is: {:#?}", kernel_as);

    // We allocate backing store for the kernel once, so we do not re-allocate it for every user
    // address space.
    kernel_as.fixate(&mut pmem)?;
    debug!("Kernel address space fixated to: {:#?}", kernel_as);

    let _user_ass = system
        .processes
        .iter()
        .map(|(_, p)| -> Result<AddressSpace, Error> {
            to_user_as(p, user_binaries, &kernel_as)?.fixated(&mut pmem)
        })
        .collect::<Result<Vec<AddressSpace>, Error>>()?;

    todo!("generate page tables and patch them into pmem");
    todo!("generate ELF boot image")
}
