use std::fmt;
use std::io;

#[derive(Debug)]
pub enum CsvError {
    Io(io::Error),
    Parse {
        line: usize,
        col: usize,
        message: String,
    },
    InvalidEscape {
        line: usize,
        col: usize,
    },
}

pub type CsvResult<T> = Result<T, CsvError>;

impl fmt::Display for CsvError {
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
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            CsvError::Io(e) => Some(e),
            _ => None,
        }
    }
}

impl From<io::Error> for CsvError {
    fn from(e: io::Error) -> Self {
        CsvError::Io(e)
    }
}