use std::cmp::{max, min};

use num::PrimInt;
use reformation::Reformation;

#[derive(Reformation, Clone, Copy, Debug, Hash, PartialEq, Eq)]
#[reformation("{lower}-{upper}")]
pub struct Range<T: PrimInt> {
    pub lower: T,
    pub upper: T,
}

impl From<(i32, i32)> for Range<i32> {
    fn from((lower, upper): (i32, i32)) -> Self {
        Range { lower, upper }
    }
}

impl From<(u64, u64)> for Range<u64> {
    fn from((lower, upper): (u64, u64)) -> Self {
        Range { lower, upper }
    }
}

impl<T: PrimInt> Range<T> {
    pub fn is_subrange_inclusive(&self, other: &Range<T>) -> bool {
        self.lower >= other.lower && self.upper <= other.upper
    }

    pub fn is_subrange_exclusive(&self, other: &Range<T>) -> bool {
        self.lower > other.lower && self.upper < other.upper
    }

    pub fn contains(&self, point: T) -> bool {
        self.lower <= point && point <= self.upper
    }

    pub fn overlap(&self, other: &Range<T>) -> bool {
        (self.lower >= other.lower && self.lower <= other.upper)
            || (self.upper <= other.upper && self.upper >= other.lower)
            || self.is_subrange_inclusive(other)
            || other.is_subrange_inclusive(self)
    }

    pub fn overlap_or_adjacent(&self, other: &Range<T>) -> bool {
        self.overlap(other)
            || self.upper == other.lower - T::one()
            || other.upper == self.lower - T::one()
    }

    // assume overlap
    pub fn merge(&self, other: &Range<T>) -> Range<T> {
        Range {
            lower: min(self.lower, other.lower),
            upper: max(self.upper, other.upper),
        }
    }
}
