#[macro_use]
extern crate anyhow;

use anyhow::Error;

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

fn main() -> Result<(), Error> {
    epoxy::main()
}
