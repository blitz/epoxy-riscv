use failure::Error;
use log::error;

use crate::address_space::AddressSpace;
use crate::phys_mem::PhysMemory;

/// A page table format.
#[derive(Debug, Clone, Copy)]
pub enum Format {
    RiscvSv32,
}

pub fn generate(
    _format: Format,
    _addr_space: &AddressSpace,
    _pmem: &mut PhysMemory,
) -> Result<u64, Error> {
    error!("TODO Implement page table generation");

    Ok(0xDEADBEEF)
}
