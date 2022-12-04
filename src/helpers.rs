use std::{
    cmp::{max, min},
    ops::Range,
};

pub trait RangeExt<T> {
    /// Is x a subset of the range
    fn is_subset(&self, x: &Range<T>) -> bool;
    /// Is x in any way intersect with the range
    fn has_intersection(&self, x: &Range<T>) -> bool;
}

impl<T: PartialOrd + Ord + Copy> RangeExt<T> for Range<T> {
    fn is_subset(&self, x: &Range<T>) -> bool {
        self.start >= x.start && self.end <= x.end
    }

    fn has_intersection(&self, x: &Range<T>) -> bool {
        let lowest = max(self.start, x.start);
        let highest = min(self.end, x.end);
        lowest <= highest
    }
}
