//! This module implements a memory abstraction. Any content rendered into this memory will be part
//! of the boot image that is generated.

use std::convert::{From, TryInto};

use crate::interval::Interval;

#[derive(Debug)]
struct Chunk {
    paddr: u64,
    data: Vec<u8>,
}

impl From<&Chunk> for Interval {
    fn from(chunk: &Chunk) -> Self {
        Interval::new_with_size(chunk.paddr, chunk.data.len().try_into().unwrap())
    }
}

/// A memory abstraction that allows reading and writing to it.
#[derive(Default, Debug)]
struct Memory {
    chunks: Vec<Chunk>,
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
    pub fn write(&mut self, paddr: u64, data: &[u8]) {
        self.chunks.push(Chunk {
            paddr,
            data: data.to_vec(),
        })
    }

    pub fn read(&self, paddr: u64, size: u64) -> Vec<u8> {
        read_rec(
            self.chunks.iter().rev(),
            Interval::new_with_size(paddr, size),
        )
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
    }
}
