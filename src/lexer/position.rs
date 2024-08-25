use std::fmt;

#[derive(Clone, PartialEq, Eq)]
pub struct Position {
    pub line: usize,
    pub col: usize,
    pub index: usize,
}

impl Position {
    pub fn new() -> Self {
        Self {
            line: 0,
            col: 0,
            index: 0,
        }
    }
}

impl fmt::Debug for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "(line: {}, col: {}, index: {})",
            self.line, self.col, self.index
        )
    }
}
