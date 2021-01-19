use elfy::types::ProgramHeaderFlags;
use elfy::Elf;
use std::convert::TryInto;
use std::fmt;
use std::iter;

use crate::constants::PAGE_SIZE;

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

#[derive(Clone, Copy)]
pub struct Permissions {
    elf_perm: ProgramHeaderFlags,
}

// This is a bit unfortunate.
impl Permissions {
    pub fn can_read(&self) -> bool {
        match self.elf_perm {
            ProgramHeaderFlags::Read => true,
            ProgramHeaderFlags::ReadWrite => true,
            ProgramHeaderFlags::ReadExecute => true,
            _ => false,
        }
    }

    pub fn can_write(&self) -> bool {
        match self.elf_perm {
            ProgramHeaderFlags::Write => true,
            ProgramHeaderFlags::ReadWrite => true,
            ProgramHeaderFlags::ReadWriteExecute => true,
            _ => false,
        }
    }

    pub fn can_execute(&self) -> bool {
        match self.elf_perm {
            ProgramHeaderFlags::Execute => true,
            ProgramHeaderFlags::ReadExecute => true,
            ProgramHeaderFlags::ReadWriteExecute => true,
            _ => false,
        }
    }
}

impl fmt::Debug for Permissions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.pad(&format!(
            "{}{}{}",
            if self.can_read() { "R" } else { " " },
            if self.can_write() { "W" } else { " " },
            if self.can_execute() { "X" } else { " " }
        ))
    }
}

#[derive(Clone)]
pub struct Mapping {
    vstart: u64,
    perm: Permissions,
    backing: Backing,
}

impl fmt::Debug for Mapping {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.pad(&format!(
            "< {:#08x} {:?}: {:#?}>",
            self.vstart, self.perm, self.backing
        ))
    }
}

impl Mapping {
    /// Return a new mapping that is a page aligned version of self. Space that needs to be added is
    /// zero padded when required.
    pub fn page_aligned(&self) -> Self {
        // The amount of bytes the start of the mapping is beyond a page boundary.
        let offset = (PAGE_SIZE - (self.vstart % PAGE_SIZE)) % PAGE_SIZE;

        // The amount of bytes missing at the end to let the mapping end on a page boundary.
        let pad_bytes = (PAGE_SIZE - ((self.backing.size() + offset) % PAGE_SIZE)) % PAGE_SIZE;

        assert_eq!((self.vstart - offset) % PAGE_SIZE, 0);
        assert_eq!((self.backing.size() + pad_bytes) % PAGE_SIZE, 0);

        Mapping {
            vstart: self.vstart - offset,
            perm: self.perm,
            backing: self.backing.moved_left(offset).extended(pad_bytes),
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
                .segments()
                .map(|s| {
                    let h = s.header();

                    Mapping {
                        vstart: h.virtual_address().try_into().unwrap(),
                        perm: Permissions {
                            elf_perm: h.flags(),
                        },
                        backing: Backing::InitializedData {
                            data: {
                                let mut d = s.data().clone();
                                d.resize(h.memory_size(), 0);
                                d
                            },
                        },
                    }
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

    /// Merge another address space into this one.
    pub fn merge_from(&mut self, o: &AddressSpace) {
        self.mappings.extend(o.iter().cloned());
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
