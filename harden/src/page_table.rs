use byteorder::{LittleEndian, WriteBytesExt};
use failure::Error;
use log::debug;
use std::convert::TryInto;

use crate::address_space::{AddressSpace, Permissions};
use crate::phys_mem::{PhysMemory, PlaceAs};

/// A page table format.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Format {
    RiscvSv32,
}

/// Turn a vector of integers into its byte representation.
fn combine(v: &[u32]) -> Vec<u8> {
    v.iter().fold(vec![], |mut acc, &v| {
        // TODO Propagate errors.
        acc.write_u32::<LittleEndian>(v).unwrap();
        acc
    })
}

fn permission_bits(perm: Permissions) -> u32 {
    (if perm.read { 1 << 1 } else { 0 })
        | (if perm.write { 1 << 2 } else { 0 })
        | (if perm.execute { 1 << 3 } else { 0 })
        | (if perm.user { 1 << 4 } else { 0 })
        | (1 << 6)              // Accessed
        | (1 << 7)              // Dirty
        | 1 // valid
}

fn pt_entry(vaddr: u64, addr_space: &AddressSpace) -> u32 {
    if let Some((paddr, perm)) = addr_space.lookup(vaddr) {
        assert_eq!(paddr & 0xFFF, 0);

        // TODO Propagate errors.
        let paddr_32: u32 = paddr.try_into().unwrap();

        (paddr_32 >> 2) | permission_bits(perm)
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
        let combined = combine(&pt_data);
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
