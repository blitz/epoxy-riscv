/// A half-open interval.
#[derive(Default, Debug, PartialEq, Eq, Clone, Copy)]
pub struct Interval {
    /// The first value of the interval.
    pub from: u64,

    /// The first non-valid value of the interval.
    pub to: u64,
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

    pub fn contains(&self, p: u64) -> bool {
        if self.empty() {
            false
        } else {
            self.from <= p && p < self.to
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

    pub fn intersection(&self, other: Interval) -> Interval {
        if self.intersects(other) {
            Interval {
                from: u64::max(self.from, other.from),
                to: u64::min(self.to, other.to),
            }
        } else {
            Interval::default()
        }
    }

    /// Returns true if the intervals overlaied over another are contiguous.
    pub fn joinable(&self, other: Interval) -> bool {
        self.adjacent(other) || self.intersects(other)
    }

    pub fn hull(&self, other: Interval) -> Interval {
        Interval {
            from: u64::min(self.from, other.from),
            to: u64::max(self.to, other.to),
        }
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
        let i5 = Interval::new_with_size(1, 2);

        assert!(Interval::default().empty());

        assert_eq!(i1, i2);
        assert_eq!(i1.size(), 5);

        assert!(Interval::new_with_size(23, 0).empty());

        assert!(i1.adjacent(i3));
        assert!(!i1.adjacent(i4));

        assert!(i1.intersects(i1));
        assert!(!i1.intersects(i3));

        assert!(i1.intersection(i3).empty());
        assert_eq!(i1.intersection(i5), i5);
    }
}
