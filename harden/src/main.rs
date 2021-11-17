#[macro_use]
extern crate failure;

use log::error;

mod address_space;
mod boot_image;
mod bump_ptr_alloc;
mod cfgfile;
mod cfgtypes;
mod codegen;
mod constants;
mod elf;
mod elf_writer;
mod epoxy;
mod framebuffer;
mod interval;
mod kernel_codegen;
mod page_table;
mod phys_mem;
mod runtypes;
mod vec_utils;

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
