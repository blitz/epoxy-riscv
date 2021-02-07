use failure::Error;
use std::convert::TryInto;
use std::fmt;
use std::iter;

use crate::constants::PAGE_SIZE;
use crate::elf::Elf;
pub use crate::elf::Permissions;
use crate::interval::Interval;
use crate::phys_mem::{PhysMemory, PlaceAs};
use crate::runtypes;

#[derive(Clone, PartialEq)]
pub enum Backing {
    // Pre-initialized data where the physical backing store location is not relevant.
    InitializedData { data: Vec<u8> },

    // A mapping to a physical memory region.
    Phys { size: u64, phys: u64 },
}

impl fmt::Debug for Backing {
    /// Custom debug format to avoid printing megabytes of binary data to the log.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Backing::InitializedData { data } => f.pad(&format!("<{:#x} bytes>", data.len())),
            Backing::Phys { size, phys } => f.pad(&format!("<Phys {:#x}+{:#x}>", phys, size)),
        }
    }
}

impl Backing {
    /// The length of the backing store in bytes.
    pub fn size(&self) -> u64 {
        match self {
            // The try_into cannot fail, because we len() returns usize and usize always fits into u64.
            Backing::InitializedData { data } => data.len().try_into().unwrap(),
            Backing::Phys { size, .. } => *size,
        }
    }

    /// Move a mapping to the left by the given number of bytes. This basically subtracts the offset
    /// from the start address.
    fn moved_left(&self, offset: u64) -> Backing {
        match self {
            Backing::InitializedData { .. } => self.clone(),
            Backing::Phys { size, phys } => Backing::Phys {
                phys: phys - offset,
                size: *size,
            },
        }
    }

    /// Return a new backing store that is extended by the given number of bytes. Content is zero-padded if possible.
    pub fn extended(&self, bytes: u64) -> Backing {
        match self {
            Backing::InitializedData { data } => Backing::InitializedData {
                data: data
                    .iter()
                    .cloned()
                    .chain(iter::repeat(0).take(bytes.try_into().unwrap()))
                    .collect(),
            },
            Backing::Phys { size, phys } => Backing::Phys {
                size: size + bytes,
                phys: *phys,
            },
        }
    }
}

#[derive(Clone)]
pub struct Mapping {
    vaddr: u64,
    perm: Permissions,
    backing: Backing,
}

impl fmt::Debug for Mapping {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.pad(&format!(
            "< {:#08x} {:?}: {:#?}>",
            self.vaddr, self.perm, self.backing
        ))
    }
}

impl Mapping {
    /// Return a new mapping that is a page aligned version of self. Space that needs to be added is
    /// zero padded when required.
    pub fn page_aligned(&self) -> Self {
        // The amount of bytes the start of the mapping is beyond a page boundary.
        let offset = (PAGE_SIZE - (self.vaddr % PAGE_SIZE)) % PAGE_SIZE;

        // The amount of bytes missing at the end to let the mapping end on a page boundary.
        let pad_bytes = (PAGE_SIZE - ((self.backing.size() + offset) % PAGE_SIZE)) % PAGE_SIZE;

        assert_eq!((self.vaddr - offset) % PAGE_SIZE, 0);
        assert_eq!((self.backing.size() + pad_bytes) % PAGE_SIZE, 0);

        Mapping {
            vaddr: self.vaddr - offset,
            perm: self.perm,
            backing: self.backing.moved_left(offset).extended(pad_bytes),
        }
    }

    pub fn size(&self) -> u64 {
        self.backing.size()
    }

    /// Return the virtual address interval covered by the mapping.
    pub fn virt_ivl(&self) -> Interval {
        Interval::new_with_size(self.vaddr, self.size())
    }
}

impl From<&runtypes::VirtualMemoryRegion> for Mapping {
    fn from(mres: &runtypes::VirtualMemoryRegion) -> Self {
        Mapping {
            vaddr: mres.virt_start,
            perm: Permissions::read_write(),
            backing: match mres.phys {
                runtypes::MemoryRegion::Phys { size, start } => Backing::Phys { phys: start, size },
                runtypes::MemoryRegion::AnonymousZeroes { size } => Backing::InitializedData {
                    data: vec![0; size.try_into().unwrap()],
                },
            },
        }
    }
}

// A architecture-neutral address space description.
#[derive(Debug, Clone)]
pub struct AddressSpace {
    mappings: Vec<Mapping>,
}

impl From<&Elf> for AddressSpace {
    /// Create an address space from an ELF binary. This converts all segments in the ELF to
    /// mappings. It ignores the physical memory addresses, so it might not be suitable for all
    /// kinds of ELFs.
    fn from(elf: &Elf) -> Self {
        AddressSpace {
            mappings: elf
                .segments
                .iter()
                .map(|s| Mapping {
                    vaddr: s.vaddr,
                    perm: s.permissions,
                    backing: Backing::InitializedData {
                        data: s.data.clone(),
                    },
                })
                .map(|m| m.page_aligned())
                .collect(),
        }
    }
}

impl IntoIterator for AddressSpace {
    type Item = <std::vec::Vec<Mapping> as IntoIterator>::Item;
    type IntoIter = <std::vec::Vec<Mapping> as IntoIterator>::IntoIter;

    fn into_iter(self) -> <Self as std::iter::IntoIterator>::IntoIter {
        self.mappings.into_iter()
    }
}

impl AddressSpace {
    /// Returns an iterator over all address space elements.
    pub fn iter(&self) -> std::slice::Iter<Mapping> {
        self.mappings.iter()
    }

    /// Look up the physical address and the mapping it belongs to.
    pub fn lookup(&self, vaddr: u64) -> Option<(u64, Permissions)> {
        let m = self
            .mappings
            .iter()
            .find(|m| m.virt_ivl().contains(vaddr))?;
        let offset = vaddr - m.vaddr;

        match &m.backing {
            Backing::Phys { phys, .. } => Some((phys + offset, m.perm)),
            _ => None,
        }
    }

    /// Look up the physical address. This is similar to `lookup`, but doesn't return the
    /// permissions as well.
    pub fn lookup_phys(&self, vaddr: u64) -> Option<u64> {
        self.lookup(vaddr).map(|(paddr, _)| paddr)
    }

    /// Merge another address space into this one.
    pub fn merge_from(&mut self, o: &AddressSpace) {
        self.mappings.extend(o.iter().cloned());
    }

    /// Fixate all initialized memory by writing it into the provided physical memory structure. Any
    /// InitializedData mapping will replaced by a Phys mapping.
    pub fn fixate(&mut self, pmem: &mut PhysMemory) -> Result<(), Error> {
        self.mappings = self
            .mappings
            .iter()
            .map(|m| -> Result<Mapping, Error> {
                match &m.backing {
                    Backing::InitializedData { data } => Ok(Mapping {
                        backing: Backing::Phys {
                            size: data.len().try_into()?,
                            phys: pmem
                                .place(
                                    &data,
                                    if m.perm.write {
                                        PlaceAs::Unique
                                    } else {
                                        PlaceAs::Shareable
                                    },
                                )
                                .ok_or_else(|| {
                                    format_err!(
                                    "Unable to fixate initialized data section at {:#x} in memory",
                                    m.vaddr
                                )
                                })?,
                        },
                        ..*m
                    }),
                    Backing::Phys { .. } => Ok(m.clone()),
                }
            })
            .collect::<Result<Vec<Mapping>, Error>>()?;
        Ok(())
    }

    /// Mark all mappings available to user code.
    pub fn make_user(&mut self) {
        for m in &mut self.mappings {
            m.perm.user = true;
        }
    }

    pub fn fixated(&self, pmem: &mut PhysMemory) -> Result<AddressSpace, Error> {
        let mut copy = self.clone();

        copy.fixate(pmem)?;
        Ok(copy)
    }

    pub fn add(&mut self, mapping: Mapping) {
        self.mappings.push(mapping)
    }

    /// Extend the address space with all mappings that the iterator produces.
    pub fn extend<T: Iterator<Item = Mapping>>(&mut self, iter: T) {
        for m in iter {
            self.add(m)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_backing() {
        let init = Backing::InitializedData { data: vec![1, 2] };

        let phys = Backing::Phys {
            phys: 0x1010,
            size: 0x0010,
        };

        // size()
        assert_eq!(init.size(), 2);
        assert_eq!(phys.size(), 0x10);

        // moved_left()
        assert_eq!(init.moved_left(1), init);
        assert_eq!(
            phys.moved_left(1),
            (Backing::Phys {
                phys: 0x100f,
                size: 0x0010,
            })
        );

        // extended()
        assert_eq!(init.extended(4).size(), 6);
        assert_eq!(phys.extended(4).size(), 0x14);
    }
}
