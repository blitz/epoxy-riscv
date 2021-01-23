use std::iter::FromIterator;

pub trait SimpleAlloc {
    /// Allocate a piece of memory from the allocator. The actual allocator type determines the
    /// alignment, but we assume page alignment is the smallest that makes sense.
    fn alloc(&mut self, size: u64) -> Option<u64>;
}

/// A simple bump pointer allocator that can be used to quickly allocate regions that never have to
/// be freed.
pub struct BumpPointerAlloc {
    current: u64,

    min_align: u64,
    end: u64,
}

fn is_power_of_two(n: u64) -> bool {
    n > 0 && (n & (n - 1)) == 0
}

impl BumpPointerAlloc {
    pub fn new(start: u64, end: u64, min_align: u64) -> Self {
        assert!(start <= end);
        assert!(min_align > 0);
        assert!(is_power_of_two(min_align));
        assert_eq!(start & (min_align - 1), 0);

        BumpPointerAlloc {
            current: start,
            min_align,
            end,
        }
    }
}

impl SimpleAlloc for BumpPointerAlloc {
    /// Allocate a region of the given size. The region will be allocated according to the alignment
    /// specified when the allocator was created.
    fn alloc(&mut self, size: u64) -> Option<u64> {
        assert_eq!(self.current & (self.min_align - 1), 0);

        let cur = self.current;
        let next_aligned = self
            .current
            .checked_add(size.checked_add(self.min_align - 1)?)?
            & !(self.min_align - 1);

        if next_aligned <= self.end {
            self.current = next_aligned;
            Some(cur)
        } else {
            None
        }
    }
}

/// A allocator that allocates from discontiguous pieces. It will use one allocator until it is
/// exhausted and then continue to the next until
pub struct ChainedAlloc<T: SimpleAlloc> {
    /// The allocators in reverse order, i.e. the first allocator to use is the last element. This
    /// helps with slowly popping them off once they become empty.
    backends: Vec<T>,
}

impl<T: SimpleAlloc> FromIterator<T> for ChainedAlloc<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self {
            // Store allocators in reverse order. This is a bit convoluted, because we don't get a
            // reversable iterator as input.
            backends: iter
                .into_iter()
                .collect::<Vec<T>>()
                .into_iter()
                .rev()
                .collect(),
        }
    }
}

impl<T: SimpleAlloc> SimpleAlloc for ChainedAlloc<T> {
    fn alloc(&mut self, size: u64) -> std::option::Option<u64> {
        match self.backends.last_mut() {
            None => None,
            Some(a) => match a.alloc(size) {
                Some(v) => Some(v),
                None => {
                    // One allocator is exhausted. Remove it and recurse.
                    self.backends.pop();
                    self.alloc(size)
                }
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alloc() {
        let mut a = BumpPointerAlloc::new(0x1000, 0x2000, 0x10);

        assert_eq!(a.alloc(0x10), Some(0x1000));
        assert_eq!(a.alloc(0x1), Some(0x1010));
        assert_eq!(a.alloc(0x1), Some(0x1020));
        assert_eq!(a.alloc(0x1000), None);
    }

    #[test]
    fn test_chained_alloc() {
        let mut a = vec![
            BumpPointerAlloc::new(0x1000, 0x1040, 0x10),
            BumpPointerAlloc::new(0x2000, 0x2020, 0x10),
        ]
        .into_iter()
        .collect::<ChainedAlloc<_>>();

        assert_eq!(a.alloc(0x10), Some(0x1000));
        assert_eq!(a.alloc(0x20), Some(0x1010));
        assert_eq!(a.alloc(0x20), Some(0x2000));
        assert_eq!(a.alloc(0x20), None);
    }
}
