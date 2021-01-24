//! A simplified ELF writer.
//!
//! Epoxy is loaded as ELF file that is only there to get memory to
//! where it needs to be in physical memory and let the boot loader
//! know where to find the kernel's entry point. So we only support a
//! tiny subset of the ELF format [1].
//!
//! As an input we get a Memory structure that describes what regions
//! of physical memory are populated. Each populated region becomes one
//! segment in the ELF. Each segment needs a PHDR and all of the PHDRs
//! are pointed to by the file header (EHDR) of the ELF file.
//!
//! The structure of the ELF file we are creating thus looks like this:
//!
//! Offset
//! 0            +----------------------------+
//!              | EHDR                       |
//!              |  points to PHDRs below     |
//! ehdr_len     +----------------------------+
//!              | PHDR 0                     |
//!              |  points to segment0        |
//!              |  initialized data          |
//!              +----------------------------+
//!              | PHDR 1                     |
//!              |  points to segment1        |
//!              |  initialized data          |
//!              +----------------------------+
//!              | ... more PHDRs ...         |
//! data_start   +----------------------------+
//!              | Segment 0 initialized data |
//!              +----------------------------+
//!              | Segment 1 initialized data |
//!              +----------------------------+
//!              | ... more segment data ...  |
//!              +----------------------------+
//!
//! [1] https://en.wikipedia.org/wiki/Executable_and_Linkable_Format

use byteorder::{BigEndian, LittleEndian, WriteBytesExt};
use failure::Error;
use std::convert::TryInto;
use std::io::Write;

use crate::phys_mem::{Chunk, PhysMemory};

#[derive(Debug, Clone, Copy)]
pub enum Format {
    Elf32,
    Elf64,
}

fn write_native<T: Write>(buf: &mut T, format: Format, value: u64) -> Result<(), Error> {
    Ok(match format {
        Format::Elf32 => buf.write_u32::<LittleEndian>(value.try_into()?)?,
        Format::Elf64 => buf.write_u64::<LittleEndian>(value)?,
    })
}

fn ehdr_len(format: Format) -> u64 {
    match format {
        Format::Elf32 => 0x34,
        Format::Elf64 => 0x40,
    }
}

fn phdr_len(format: Format) -> u64 {
    match format {
        Format::Elf32 => 0x20,
        Format::Elf64 => 0x38,
    }
}

fn shdr_len(format: Format) -> u64 {
    match format {
        Format::Elf32 => 0x28,
        Format::Elf64 => 0x40,
    }
}

/// The offset at which the segment data is serialized into the resulting ELF. This is right after
/// all headers.
fn data_start(format: Format, chunks: &[Chunk]) -> u64 {
    let clen: u64 = chunks.len().try_into().unwrap();

    ehdr_len(format) + phdr_len(format) * clen
}

fn write_ehdr<T: Write>(
    buf: &mut T,
    format: Format,
    entry: u64,
    phdr_count: usize,
) -> Result<(), Error> {
    buf.write_u32::<BigEndian>(0x7F454c46)?; // Magic

    buf.write_all(&[
        match format {
            Format::Elf32 => 1,
            Format::Elf64 => 2,
        },
        1, // Little-Endian
        1, // Version
        0, // System-V ABI
    ])?;

    buf.write_u64::<BigEndian>(0)?;

    // The fields below use the endianness specified above.

    buf.write_u16::<LittleEndian>(2)?; // Exectuable
    buf.write_u16::<LittleEndian>(0xf3)?; // Machine: RISC-V
    buf.write_u32::<LittleEndian>(1)?; // Version

    write_native(buf, format, entry)?;
    write_native(buf, format, ehdr_len(format))?; // Start of Phdrs
    write_native(buf, format, 0)?; // Start of Shdrs (we have none)

    buf.write_u32::<LittleEndian>(0)?; // Flags
    buf.write_u16::<LittleEndian>(ehdr_len(format).try_into()?)?;
    buf.write_u16::<LittleEndian>(phdr_len(format).try_into()?)?;

    buf.write_u16::<LittleEndian>(phdr_count.try_into()?)?;
    buf.write_u16::<LittleEndian>(shdr_len(format).try_into()?)?;

    buf.write_u16::<LittleEndian>(0)?; // Shdr count
    buf.write_u16::<LittleEndian>(0)?; // stridx

    Ok(())
}

pub fn write<T: Write>(
    buf: &mut T,
    format: Format,
    entry: u64,
    pmem: &PhysMemory,
) -> Result<(), Error> {
    let chunks = pmem.chunks();

    write_ehdr(buf, format, entry, chunks.len())?;

    // TODO

    Ok(())
}
