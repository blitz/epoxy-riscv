#[macro_use]
extern crate failure;

use log::error;

mod cfgfile;
mod cfgtypes;
mod codegen;
mod epoxy;
mod framebuffer;
mod runtypes;

fn main() {
    std::process::exit(match epoxy::main() {
        Ok(_) => 0,
        Err(e) => {
            error!("Exiting because of the following chain of errors:");
            for (i, cause) in e.iter_chain().enumerate() {
                error!("Error #{}: {}", i, cause);
            }
            1
        }
    });
}
