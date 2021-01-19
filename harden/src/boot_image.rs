use elfy::Elf;
use failure::{Error, ResultExt};
use log::{debug, info};
use std::path::{Path, PathBuf};

use crate::address_space::AddressSpace;
use crate::runtypes;

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
    let kernel_as = AddressSpace::from(&kernel_elf);
    debug!("Kernel address space is: {:#?}", kernel_as);

    let _user_ass = system
        .processes
        .iter()
        .map(|(_, p)| to_user_as(p, user_binaries, &kernel_as))
        .collect::<Result<Vec<AddressSpace>, Error>>()?;

    todo!()
}
