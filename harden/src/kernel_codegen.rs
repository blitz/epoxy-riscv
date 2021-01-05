use failure::Error;

use crate::runtypes;

/// Generate the C++ code for the kernel configuration.
pub fn generate_cpp(_system: &runtypes::Configuration) -> Result<String, Error> {
    todo!()
}

/// Generate the header file for the kernel configuration.
pub fn generate_hpp(_system: &runtypes::Configuration) -> Result<String, Error> {
    todo!()
}
