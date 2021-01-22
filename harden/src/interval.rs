/// A half-open interval.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Interval {
    /// The first value of the interval.
    from: u64,

    /// The first non-valid value of the interval.
    to: u64,
}

impl Interval {
    pub fn new_with_size(start: u64, size: u64) -> Interval {
        Interval {
            from: start,
            to: start + size,
        }
    }

    /// Checks whether an interval is empty.
    pub fn empty(&self) -> bool {
        self.from >= self.to
    }

    /// Returns the number of elements in the interval.
    pub fn size(&self) -> u64 {
        if self.empty() {
            0
        } else {
            self.to - self.from
        }
    }

    /// Returns true, if both intervals are adjacent. Empty intervals are adjacent to nothing.
    pub fn adjacent(&self, other: Interval) -> bool {
        if self.empty() || other.empty() {
            false
        } else {
            self.to == other.from || other.to == self.from
        }
    }

    pub fn intersects(&self, other: Interval) -> bool {
        if self.empty() || other.empty() {
            false
        } else {
            let begins_before = self.to <= other.from;
            let starts_after = other.to <= self.from;

            !(begins_before || starts_after)
        }
    }

    pub fn joinable(&self, other: Interval) -> bool {
        self.adjacent(other) || self.intersects(other)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interval() {
        let i1 = Interval { from: 0, to: 5 };
        let i2 = Interval::new_with_size(0, 5);
        let i3 = Interval { from: 5, to: 6 };
        let i4 = Interval { from: 5, to: 5 };

        assert_eq!(i1, i2);
        assert_eq!(i1.size(), 5);

        assert!(Interval::new_with_size(23, 0).empty());

        assert!(i1.adjacent(i3));
        assert!(!i1.adjacent(i4));

        assert!(i1.intersects(i1));
        assert!(!i1.intersects(i3));
    }
}
