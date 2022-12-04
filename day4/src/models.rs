//! Models to encapsulate the logic of the range operations.

/// Represents a range between 2 integers.
pub struct Range {
    start: u32,
    end: u32,
}

impl Range {
    /// Check whether the range contains another range.
    pub fn contains(&self, other: &Range) -> bool {
        self.start >= other.start && self.end <= other.end
    }

    /// Check whether the range overlaps with another range.
    pub fn overlaps(&self, other: &Range) -> bool {
        !(self.end < other.start || self.start > other.end)
    }
}

impl TryFrom<&str> for Range {
    type Error = &'static str;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        let (start, end) = input.split_once('-').ok_or("failed to split into range")?;

        Ok(Self {
            start: start.parse().map_err(|_| "failed to parse start")?,
            end: end.parse().map_err(|_| "failed to parse end")?,
        })
    }
}
