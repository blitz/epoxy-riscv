use elfy::types::ProgramHeaderFlags;
use elfy::Elf;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub enum Backing {
    // Pre-initialized data where the physical backing store location is not relevant.
    InitializedData { data: Vec<u8> },

    // Zero-initialized data where the physical backing store location is not relevant.
    Zeroes { size: u64 },

    // A mapping to a physical memory region.
    Phys { size: u64, phys: u64 },
}

impl Backing {
    pub fn size(&self) -> u64 {
        match self {
            // The try_into cannot fail, because we len() returns usize and usize always fits into u64.
            Backing::InitializedData { data } => data.len().try_into().unwrap(),
            Backing::Zeroes { size } => *size,
            Backing::Phys { size, .. } => *size,
        }
    }
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct Mapping {
    vstart: u64,
    perm: Permissions,
    backing: Backing,
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
