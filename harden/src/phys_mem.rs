//! This module implements a memory abstraction. Any content rendered into this memory will be part
//! of the boot image that is generated.

use log::debug;
use std::collections::HashMap;
use std::convert::{From, TryInto};

use crate::bump_ptr_alloc::{BumpPointerAlloc, ChainedAlloc, SimpleAlloc};
use crate::interval::Interval;

#[derive(Debug, Copy, Clone)]
pub enum PlaceAs {
    /// Memory placed as unique needs to be put into a previously unused piece of physical memory.
    Unique,

    /// Memory placed with this type is considered immutable and can be reused for other placements.
    Shareable,
}

#[derive(Debug)]
pub struct Chunk {
    pub paddr: u64,
    pub data: Vec<u8>,
}

impl From<&Chunk> for Interval {
    fn from(chunk: &Chunk) -> Self {
        Interval::new_with_size(chunk.paddr, chunk.data.len().try_into().unwrap())
    }
}

/// A memory abstraction that allows reading and writing to it.
#[derive(Default, Debug)]
struct Memory {
    pub chunks: Vec<Chunk>,
}

/// A representation of physical memory as it will be written into the boot image.
pub struct PhysMemory {
    memory: Memory,

    /// A allocator for free physical space.
    free_memory: ChainedAlloc<BumpPointerAlloc>,

    /// A map from already placed memory content to the location where it is located in memory.
    ///
    /// TODO: Note that this is extremely primitive and no attempt has been made to optimize for
    /// sharing at smaller granularities than whole ELF segments.
    shareable_memory: HashMap<Vec<u8>, u64>,
}

/// A recursive helper function for `Memory::read()`.
fn read_rec<'a, I>(mut iter: I, pivl: Interval) -> Vec<u8>
where
    I: Iterator<Item = &'a Chunk> + Clone,
{
    if pivl.empty() {
        vec![]
    } else {
        match iter.next() {
            None => vec![0; pivl.size().try_into().unwrap()],
            Some(chunk) => {
                let chunk_ivl: Interval = chunk.into();
                let intersects = pivl.intersects(chunk_ivl);
                let intersection = pivl.intersection(chunk_ivl);

                if intersects && pivl.from < chunk_ivl.from {
                    vec![
                        read_rec(
                            iter.clone(),
                            Interval {
                                from: pivl.from,
                                to: chunk_ivl.from,
                            },
                        ),
                        chunk
                            .data
                            .iter()
                            .take(intersection.size().try_into().unwrap())
                            .copied()
                            .collect::<Vec<u8>>(),
                        read_rec(
                            iter.clone(),
                            Interval {
                                from: intersection.to,
                                to: pivl.to,
                            },
                        ),
                    ]
                    .concat()
                } else if intersects {
                    vec![
                        chunk
                            .data
                            .iter()
                            .skip((pivl.from - chunk_ivl.from).try_into().unwrap())
                            .take(intersection.size().try_into().unwrap())
                            .copied()
                            .collect::<Vec<u8>>(),
                        read_rec(
                            iter.clone(),
                            Interval {
                                from: intersection.to,
                                to: pivl.to,
                            },
                        ),
                    ]
                    .concat()
                } else {
                    // No intersection. Continue recursion.
                    read_rec(iter, pivl)
                }
            }
        }
    }
}

impl Memory {
    fn write(&mut self, paddr: u64, data: &[u8]) {
        self.chunks.push(Chunk {
            paddr,
            data: data.to_vec(),
        })
    }

    fn read(&self, paddr: u64, size: u64) -> Vec<u8> {
        read_rec(
            self.chunks.iter().rev(),
            Interval::new_with_size(paddr, size),
        )
    }

    /// Simplify the internal representation by combining all previous writes.
    fn flattened(&self) -> Memory {
        let mut all_ivls = self
            .chunks
            .iter()
            .map(|c| c.into())
            .collect::<Vec<Interval>>();

        all_ivls.sort_by(|a, b| a.from.cmp(&b.from));

        // The list of all intervals that contain data.
        let joined_ivls = all_ivls
            .into_iter()
            .fold(vec![], |mut acc: Vec<Interval>, c| {
                if let Some(last_ivl) = acc.pop() {
                    if last_ivl.joinable(c) {
                        acc.push(last_ivl.hull(c));
                    } else {
                        acc.push(last_ivl);
                        acc.push(c);
                    }
                } else {
                    acc.push(c);
                };

                acc
            });

        // Just re-read the populated intervals to join all underlying chunks.
        Memory {
            chunks: joined_ivls
                .iter()
                .map(|i| Chunk {
                    paddr: i.from,
                    data: self.read(i.from, i.size()),
                })
                .collect(),
        }
    }

    fn size(&self) -> u64 {
        self.chunks
            .iter()
            .map(|c| c.data.len())
            .sum::<usize>()
            .try_into()
            .unwrap()
    }
}

impl PhysMemory {
    pub fn new(free_memory: ChainedAlloc<BumpPointerAlloc>) -> PhysMemory {
        PhysMemory {
            memory: Memory::default(),
            shareable_memory: HashMap::default(),
            free_memory,
        }
    }

    /// Writes memory to a specific location in physical memory.
    pub fn write(&mut self, paddr: u64, data: &[u8]) {
        self.memory.write(paddr, data)
    }

    pub fn place_unique(&mut self, data: &[u8]) -> Option<u64> {
        let addr = self.free_memory.alloc(data.len().try_into().unwrap())?;

        self.write(addr, data);
        Some(addr)
    }

    pub fn place_shareable(&mut self, data: &[u8]) -> Option<u64> {
        self.shareable_memory
            .get(data)
            .map(|v| {
                debug!("Reusing {:#x} bytes at {:#x}.", data.len(), *v);
                *v
            })
            .or_else(|| {
                let newly_written = self.place_unique(data)?;

                self.shareable_memory.insert(data.to_vec(), newly_written);
                Some(newly_written)
            })
    }

    /// Places data at a page aligned and free location in physical memory. Returns the address at
    /// which it was written.
    ///
    /// When `ptype` is `PlaceAs::Shareable` memory can be de-duplicated. Placing the same shareable
    /// data twice will result in the same address being returned. This is useful for read-only
    /// memory to save space.
    pub fn place(&mut self, data: &[u8], ptype: PlaceAs) -> Option<u64> {
        match ptype {
            PlaceAs::Shareable => self.place_shareable(data),
            PlaceAs::Unique => self.place_unique(data),
        }
    }

    /// Reads memory from physical memory. Returns zeros for locations that have never been written
    /// before.
    #[allow(dead_code)]
    pub fn read(&self, paddr: u64, size: u64) -> Vec<u8> {
        self.memory.read(paddr, size)
    }

    /// Return a list of memory chunks.
    pub fn chunks(&self) -> Vec<Chunk> {
        self.memory.flattened().chunks
    }

    /// Return the amount of memory stored so far.
    pub fn size(&self) -> u64 {
        self.memory.size()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory() {
        let mut m = Memory::default();

        assert_eq!(m.read(0x0000, 0), vec![]);
        assert_eq!(m.read(0x0000, 2), vec![0, 0]);

        m.write(0x1000, &[1, 2, 3]);
        assert_eq!(m.read(0x1000, 3), vec![1, 2, 3]);
        assert_eq!(m.read(0x1002, 1), vec![3]);
        assert_eq!(m.read(0x1002, 2), vec![3, 0]);
        assert_eq!(m.read(0x0FFF, 2), vec![0, 1]);
        assert_eq!(m.read(0x0FFF, 5), vec![0, 1, 2, 3, 0]);

        m.write(0x0FFF, &[4]);
        assert_eq!(m.read(0x0FFF, 2), vec![4, 1]);

        m.write(0x0FFF, &[7, 8]);
        assert_eq!(m.read(0x0FFF, 3), vec![7, 8, 2]);

        let flattened = m.flattened();
        assert_eq!(flattened.read(0x0fff, 3), vec![7, 8, 2]);
    }
}
