use failure::Error;
use log::debug;
use std::convert::TryInto;

use crate::address_space::{AddressSpace, Permissions};
use crate::phys_mem::{PhysMemory, PlaceAs};
use crate::vec_utils::vec_u32_to_bytes;

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

fn pt_entry(vaddr: u64, addr_space: &AddressSpace) -> u32 {
    if let Some((paddr, perm)) = addr_space.lookup(vaddr) {
        assert_eq!(paddr & 0xFFF, 0);

        // TODO Propagate errors.
        let paddr_32: u32 = paddr.try_into().unwrap();

        (paddr_32 >> 2) | u32::from(permission_bits(perm))
    } else {
        0
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
) -> Option<u32> {
    let pt_data = &(0..1024)
        .into_iter()
        .map(|pt_index| vaddr + (pt_index << (12 + (level * 10))))
        .map(|vaddr| {
            if level == 0 {
                pt_entry(vaddr, addr_space)
            } else {
                pt_next(page_table(pmem, level - 1, vaddr, addr_space))
            }
        })
        .collect::<Vec<u32>>();

    if pt_data.iter().all(|&v| v == 0) {
        None
    } else {
        let combined = vec_u32_to_bytes(&pt_data);
        let phys = pmem.place(&combined, PlaceAs::Shareable)?;

        assert_eq!(combined.len(), 4096);
        debug!("Allocated page table at phys {:#x}", phys);

        Some(phys.try_into().unwrap())
    }
}

pub fn generate(
    format: Format,
    addr_space: &AddressSpace,
    pmem: &mut PhysMemory,
) -> Result<u64, Error> {
    assert_eq!(format, Format::RiscvSv32);

    let root_pt: u64 = page_table(pmem, 1, 0, addr_space)
        .ok_or_else(|| format_err!("Failed to allocate memory for page tables"))?
        .into();

    // Turn the page table pointer into a valid SATP value for Sv32.
    let satp = (root_pt >> 12) | 1 << 31;

    debug!("User process SATP is {:#x}", satp);
    Ok(satp)
}
