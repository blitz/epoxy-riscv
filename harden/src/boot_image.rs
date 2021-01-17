use failure::Error;
use std::path::Path;

use crate::runtypes;

pub fn generate(
    _system: &runtypes::Configuration,
    _kernel_binary: &Path,
    _user_binaries: &Path,
) -> Result<(), Error> {
    todo!()
}
