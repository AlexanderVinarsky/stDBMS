use std::fmt;

#[derive(Debug)]
pub enum DbError {
    InvalidMagic {expected: u8, found: u8},
    InvalidPageCount(u8),
    Io(std::io::Error),
    StringConversion(String),
    InvalidInput {expected: String, found: usize},
}

impl From<std::io::Error> for DbError {
    fn from(error: std::io::Error) -> Self {
        DbError::Io(error)
    }
}

impl fmt::Display for DbError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DbError::InvalidMagic { expected, found }   => write!(f, "Invalid magic number: expected {}, found {}", expected, found),
            DbError::InvalidPageCount(count)            => write!(f, "Invalid page count: {}", count),
            DbError::Io(err)                            => write!(f, "IO error: {}", err),
            DbError::StringConversion(msg)              => write!(f, "String conversion error: {}", msg),
            DbError::InvalidInput { expected, found }   => write!(f, "Page name '{}' exceeds maximum length of {}", expected, found),
        }
    }
}

impl std::error::Error for DbError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            DbError::Io(err) => Some(err),
            _ => None,
        }
    }
}

pub type Result<T> = std::result::Result<T, DbError>;