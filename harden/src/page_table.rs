use anyhow::Error;
use log::debug;
use std::convert::TryInto;

use crate::address_space::{AddressSpace, Permissions};
use crate::phys_mem::{PhysMemory, PlaceAs};
use crate::vec_utils::vec_u32_to_bytes;

/// Errors from page table generation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PageTableError {
    /// A physical address was not mappable in the page table.
    ///
    /// This can happen when a 32-bit page table is created, but the physical address is beyond 4G.
    PhysAddressNotMappable { paddr: u64 },

    /// A page table was allocated at a place where we cannot point to it.
    ///
    /// This is in an internal error, because we cannot ask for below-4G memory yet.
    IllegalPageTablePlacement { paddr: u64 },

    /// We failed to allocate backing storage for page tables.
    MemoryAllocationFailed,
}

impl std::fmt::Display for PageTableError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PageTableError::PhysAddressNotMappable { paddr } => write!(
                f,
                "Physical address {:#x} is not representable in the page table.",
                paddr
            ),
            PageTableError::IllegalPageTablePlacement { paddr } => write!(
                f,
                "Internal error: A page table was allocated at {:#x}, but we cannot use this address.",
                paddr
            ),
            PageTableError::MemoryAllocationFailed => write!(f, "Failed to allocate memory for page table structures."),
        }
    }
}

impl std::error::Error for PageTableError {}

/// A page table format.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Format {
    RiscvSv32,
}

const PTE_V: u8 = 1 << 0;
const PTE_R: u8 = 1 << 1;
const PTE_W: u8 = 1 << 2;
const PTE_X: u8 = 1 << 3;
const PTE_U: u8 = 1 << 4;
const PTE_A: u8 = 1 << 6;
const PTE_D: u8 = 1 << 7;

fn permission_bits(perm: Permissions) -> u8 {
    (if perm.read { PTE_R } else { 0 })
        | (if perm.write { PTE_W } else { 0 })
        | (if perm.execute { PTE_X } else { 0 })
        | (if perm.user { PTE_U } else { 0 })
        | PTE_A
        | PTE_D
        | PTE_V
}

fn pt_entry(vaddr: u64, addr_space: &AddressSpace) -> Result<u32, PageTableError> {
    if let Some((paddr, perm)) = addr_space.lookup(vaddr) {
        assert_eq!(paddr & 0xFFF, 0);

        let paddr_32: u32 = paddr
            .try_into()
            .map_err(|_| PageTableError::PhysAddressNotMappable { paddr })?;

        Ok((paddr_32 >> 2) | u32::from(permission_bits(perm)))
    } else {
        Ok(0)
    }
}

fn pt_next(pt: Option<u32>) -> u32 {
    if let Some(phys) = pt {
        phys >> 2 | 1
    } else {
        0
    }
}

/// Generate a page table at the given level. Level counts down with being the leaf.
///
/// TODO This is very inefficient, because we iterate through all possible pages.
fn page_table(
    pmem: &mut PhysMemory,
    level: u32,
    vaddr: u64,
    addr_space: &AddressSpace,
) -> Result<Option<u32>, PageTableError> {
    let pt_data = &(0..1024)
        .into_iter()
        .map(|pt_index| vaddr + (pt_index << (12 + (level * 10))))
        .map(|vaddr| -> Result<u32, PageTableError> {
            if level == 0 {
                pt_entry(vaddr, addr_space)
            } else {
                Ok(pt_next(page_table(pmem, level - 1, vaddr, addr_space)?))
            }
        })
        .collect::<Result<Vec<u32>, PageTableError>>()?;

    if pt_data.iter().all(|&v| v == 0) {
        Ok(None)
    } else {
        let combined = vec_u32_to_bytes(&pt_data);
        let phys = pmem
            .place(&combined, PlaceAs::Shareable)
            .ok_or(PageTableError::MemoryAllocationFailed)?;

        assert_eq!(combined.len(), 4096);
        debug!("Allocated page table at phys {:#x}", phys);

        Ok(Some(phys.try_into().map_err(|_| {
            PageTableError::IllegalPageTablePlacement { paddr: phys }
        })?))
    }
}

pub fn generate(
    format: Format,
    addr_space: &AddressSpace,
    pmem: &mut PhysMemory,
) -> Result<u64, Error> {
    assert_eq!(format, Format::RiscvSv32);

    let root_pt: u64 = page_table(pmem, 1, 0, addr_space)?
        .expect("We should have at least one mapping?")
        .into();

    // Turn the page table pointer into a valid SATP value for Sv32.
    let satp = (root_pt >> 12) | 1 << 31;

    debug!("User process SATP is {:#x}", satp);
    Ok(satp)
}
