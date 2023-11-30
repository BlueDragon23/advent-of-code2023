use std::cmp::{max, min};

use reformation::Reformation;

#[derive(Reformation, Clone, Copy, Debug, Hash, PartialEq, Eq)]
#[reformation("{lower}-{upper}")]
pub struct Range {
    pub lower: i32,
    pub upper: i32,
}

impl From<(i32, i32)> for Range {
    fn from((lower, upper): (i32, i32)) -> Self {
        Range { lower, upper }
    }
}

impl Range {
    pub fn is_subrange_inclusive(&self, other: &Range) -> bool {
        self.lower >= other.lower && self.upper <= other.upper
    }
    pub fn is_subrange_exclusive(&self, other: &Range) -> bool {
        self.lower > other.lower && self.upper < other.upper
    }

    pub fn overlap(&self, other: &Range) -> bool {
        (self.lower >= other.lower && self.lower <= other.upper)
            || (self.upper <= other.upper && self.upper >= other.lower)
            || self.is_subrange_inclusive(other)
            || other.is_subrange_inclusive(self)
    }

    pub fn overlap_or_adjacent(&self, other: &Range) -> bool {
        self.overlap(other) || self.upper == other.lower - 1 || other.upper == self.lower - 1
    }

    // assume overlap
    pub fn merge(&self, other: &Range) -> Range {
        Range {
            lower: min(self.lower, other.lower),
            upper: max(self.upper, other.upper),
        }
    }
}
