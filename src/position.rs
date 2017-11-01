use std::cmp::Ordering;
use std::fmt;

#[derive(Clone, PartialEq, Eq)]
pub struct Position {
    pub line: u64,
    pub column: u64,
}

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Position) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Position {
    fn cmp(&self, other: &Position) -> Ordering {
        match self.line.cmp(&other.line) {
            Ordering::Equal => self.column.cmp(&other.column),
            x => x,
        }
    }
}

impl fmt::Debug for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Position{{ line: {}, column: {} }}",
            self.line,
            self.column
        )
    }
}
