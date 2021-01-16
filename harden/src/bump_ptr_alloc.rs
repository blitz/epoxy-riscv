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

    /// Allocate a region of the given size. The region will be allocated according to the alignment
    /// specified when the allocator was created.
    pub fn alloc(&mut self, size: u64) -> Option<u64> {
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
}
