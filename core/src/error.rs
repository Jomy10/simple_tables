use std::fmt::{Debug, Display, Formatter};

pub enum TableErrorKind {
    CouldNotRemove
}

impl Debug for TableErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CouldNotRemove => write!(f, "CouldNotRemove")
        }
    }
}

impl Display for TableErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub struct TableError {
    pub kind: TableErrorKind,
    pub message: String
}

impl Display for TableError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Debug for TableError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.kind, self.message)
    }
}

impl std::error::Error for TableError {}