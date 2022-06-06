use std::fmt::{Display, Formatter};
use std::ops::Range;

/// Represents a segment of source text, source[lo..hi]
#[derive(Debug, Copy, Clone)]
pub struct Span {
    lo: usize,
    hi: usize,
}

impl Span {
    pub fn new(lo: usize, hi: usize) -> Self {
        Self { lo, hi }
    }

    pub fn union(&self, other: &Span) -> Span {
        Span {
            lo: usize::min(self.lo, other.lo),
            hi: usize::max(self.hi, other.hi),
        }
    }

    pub fn range(&self) -> Range<usize> {
        self.lo..self.hi
    }
}

impl Display for Span {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.hi - self.lo {
            0 | 1 => write!(f, "{}", self.lo),
            _ => write!(f, "{}..{}", self.lo, self.hi),
        }
    }
}
