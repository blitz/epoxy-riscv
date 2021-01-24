//! Abstract the underlying ELF libary and expose the simple bit of functionality we need.

use failure::Error;
use goblin::Object;
use std::collections::BTreeMap;
use std::convert::TryInto;
use std::fs;
use std::path::Path;
use std::fmt;

/// Permissions for memory regions.
#[derive(PartialEq, Eq, Clone, Copy)]
pub struct Permissions {
    pub read: bool,
    pub write: bool,
    pub execute: bool,
    pub user: bool,
}

impl Permissions {
    pub fn read_write() -> Permissions {
        Permissions {
            read: true,
            write: true,
            execute: false,
            user: false,
        }
    }
}

impl fmt::Debug for Permissions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.pad(&format!(
            "{}{}{}{}",
            if self.read { "R" } else { " " },
            if self.write { "W" } else { " " },
            if self.execute { "X" } else { " " },
            if self.user { "U" } else { " " }
        ))
    }
}

impl From<&goblin::elf::program_header::ProgramHeader> for Permissions {
    fn from(ph: &goblin::elf::program_header::ProgramHeader) -> Self {
        Permissions {
            read: ph.is_read(),
            write: ph.is_write(),
            execute: ph.is_executable(),

            // This is a good default to avoid sadness. If we get this wrong, user code cannot
            // access it.
            user: false
        }
    }
}

#[derive(Debug, Clone)]
pub struct Segment {
    pub permissions: Permissions,
    pub vaddr: u64,
    pub paddr: u64,

    pub data: Vec<u8>,
}

type SymbolMap = BTreeMap<String, u64>;

pub struct Elf {
    /// Entry point of the ELF.
    pub entry: u64,

    pub segments: Vec<Segment>,
    pub symbols: SymbolMap,
}

fn elf_subslice(data: &[u8], offset: u64, filesz: u64, memsz: u64) -> Result<Vec<u8>, Error> {
    if memsz < filesz {
        Err(format_err!(
            "Invalid ELF segment: filesz {} vs memsz {}",
            filesz,
            memsz
        ))
    } else {
        let file_slice: &[u8] = &data[offset.try_into()?..(offset + filesz).try_into()?];
        let zeros: &[u8] = &vec![0; (memsz - filesz).try_into()?];

        // TODO As the integers are potentially wrong, this needs real input validation and checks
        // for integer overflow.
        Ok([file_slice, zeros].concat())
    }
}

fn elf_symbols(elf: &goblin::elf::Elf) -> Result<SymbolMap, Error> {
    elf.syms
        .iter()
        .map(|s| -> Result<(String, u64), Error> {
            let name = elf
                .strtab
                .get(s.st_name)
                .ok_or_else(|| format_err!("Failed to find symbol name in ELF"))??;

            Ok((name.to_string(), s.st_value))
        })
        .collect::<Result<SymbolMap, Error>>()
}

impl Elf {
    pub fn new(path: &Path) -> Result<Elf, Error> {
        let data = fs::read(path)?;

        match Object::parse(&data)? {
            Object::Elf(elf) => Ok(Elf {
                entry: elf.entry,
                segments: elf
                    .program_headers
                    .iter()
                    .filter(|ph| ph.p_type == goblin::elf::program_header::PT_LOAD)
                    .map(|ph| -> Result<Segment, Error> {
                        Ok(Segment {
                            permissions: ph.into(),
                            vaddr: ph.p_vaddr,
                            paddr: ph.p_paddr,

                            data: elf_subslice(&data, ph.p_offset, ph.p_filesz, ph.p_memsz)?,
                        })
                    })
                    .collect::<Result<Vec<Segment>, Error>>()?,

                symbols: elf_symbols(&elf)?,
            }),
            _ => Err(format_err!(
                "File format of {} not recognized",
                path.display()
            )),
        }
    }
}
