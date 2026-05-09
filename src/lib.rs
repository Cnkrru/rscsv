//! # rscsv
//!
//! A simple, zero-dependency CSV library for Rust built using the standard library.
//!
//! ## Features
//!
//! - **Zero Dependencies**: Pure standard library implementation
//! - **RFC 4180 Compliant**: Full support for quoted fields and escaping
//! - **Flexible Configuration**: Custom delimiters, quotes, escape characters, comments
//! - **Header Support**: Automatic header detection and retrieval
//! - **Flexible Mode**: Allow varying column counts per row
//! - **Comment Lines**: Skip lines starting with a configurable comment character
//! - **Generic Writer**: Write records from any type implementing `AsRef<str>`
//!
//! ## Quick Start
//!
//! ```rust
//! use rscsv::{CsvReader, CsvConfig, CsvWriter};
//!
//! // Reading CSV
//! let mut reader = CsvReader::from_reader("name,age\nAlice,30\nBob,25\n".as_bytes());
//! let records = reader.read_all().unwrap();
//! assert_eq!(records.len(), 2);
//!
//! // Writing CSV
//! let mut buf = Vec::new();
//! let mut writer = CsvWriter::from_writer(&mut buf);
//! writer.write_record(&["header1", "header2"]).unwrap();
//! writer.write_record(&["val1", "val2"]).unwrap();
//! ```
//!
//! ## Custom Configuration
//!
//! ```rust
//! use rscsv::{CsvConfig, CsvReader};
//!
//! let config = CsvConfig::builder()
//!     .delimiter(b';')
//!     .has_headers(true)
//!     .comment(b'#')
//!     .build();
//!
//! let data = "# comment\nname;age\nAlice;30\n".as_bytes();
//! let mut reader = CsvReader::from_reader(data).with_config(config);
//!
//! assert_eq!(reader.headers().unwrap(), &["name", "age"]);
//! let records = reader.read_all().unwrap();
//! assert_eq!(records.len(), 1);
//! ```

mod config;
mod error;
mod parser;
mod reader;
mod record;
mod writer;

pub use config::{CsvConfig, CsvConfigBuilder};
pub use error::{CsvError, CsvResult};
pub use parser::CsvParser;
pub use reader::{CsvReader, FileReader};
pub use record::StringRecord;
pub use writer::{CsvWriter, FileWriter};