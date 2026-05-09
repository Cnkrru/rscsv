//! Error handling module
//!
//! Defines error types and result type alias for the CSV library.

use std::fmt;
use std::io;

/// CSV error type
///
/// Represents all possible errors that can occur during CSV operations.
///
/// # Variants
///
/// * `Io` - I/O error from file operations
/// * `Parse` - Parse error with line/column position and message
/// * `InvalidEscape` - Invalid escape sequence with line/column position
#[derive(Debug)]
pub enum CsvError {
    /// I/O error wrapping `std::io::Error`
    Io(io::Error),
    /// Parse error with position information
    Parse {
        /// Line number where the error occurred
        line: usize,
        /// Column number where the error occurred
        col: usize,
        /// Error description
        message: String,
    },
    /// Invalid escape sequence in quoted field
    InvalidEscape {
        /// Line number where the error occurred
        line: usize,
        /// Column number where the error occurred
        col: usize,
    },
}

/// Result type alias for CSV operations
pub type CsvResult<T> = Result<T, CsvError>;

impl fmt::Display for CsvError {
    /// Formats the error as a human-readable string
    ///
    /// ```rust
    /// use rscsv::{CsvError, CsvResult};
    ///
    /// let err = CsvError::Parse { line: 1, col: 5, message: "unexpected character".into() };
    /// assert_eq!(err.to_string(), "解析错误 (行 1, 列 5): unexpected character");
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CsvError::Io(e) => write!(f, "IO 错误: {}", e),
            CsvError::Parse { line, col, message } => {
                write!(f, "解析错误 (行 {}, 列 {}): {}", line, col, message)
            }
            CsvError::InvalidEscape { line, col } => {
                write!(f, "无效转义 (行 {}, 列 {})", line, col)
            }
        }
    }
}

impl std::error::Error for CsvError {
    /// Returns the underlying I/O error source, if applicable
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            CsvError::Io(e) => Some(e),
            _ => None,
        }
    }
}

impl From<io::Error> for CsvError {
    /// Converts an I/O error into a `CsvError::Io`
    fn from(e: io::Error) -> Self {
        CsvError::Io(e)
    }
}