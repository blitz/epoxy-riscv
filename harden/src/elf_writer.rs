// TODO Write module to write an ELF comprised of arbitrary segments.

use log::{error};

use crate::phys_mem::PhysMemory;

#[derive(Debug, Clone, Copy)]
pub enum Format {
    Elf32, Elf64
}

pub fn write(format: Format, entry: u64, pmem: &PhysMemory) -> Vec<u8> {
    error!("TODO Implement writing ELF files");
    
    vec![]
}
