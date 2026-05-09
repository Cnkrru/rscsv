//! CSV writer module
//!
//! Provides a CSV writer with automatic quoting and support for
//! generic record types.

use crate::config::CsvConfig;
use crate::error::CsvResult;
use crate::record::StringRecord;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

/// CSV writer
///
/// Writes CSV data to any [`Write`] target with automatic quoting and escaping.
///
/// # Examples
///
/// ```rust
/// use rscsv::CsvWriter;
///
/// let mut buf = Vec::new();
/// let mut writer = CsvWriter::from_writer(&mut buf);
/// writer.write_record(&["name", "age"]).unwrap();
/// writer.write_record(&["Alice", "30"]).unwrap();
///
/// let result = String::from_utf8(buf).unwrap();
/// assert_eq!(result, "name,age\nAlice,30\n");
/// ```
pub struct CsvWriter<W: Write> {
    writer: W,
    config: CsvConfig,
}

/// Convenience type alias for writing CSV to a file
pub type FileWriter = CsvWriter<BufWriter<File>>;

impl FileWriter {
    /// Create a new CSV file for writing
    ///
    /// # Parameters
    ///
    /// * `path` - Path to the output file
    pub fn create<P: AsRef<Path>>(path: P) -> CsvResult<Self> {
        let file = File::create(path)?;
        let writer = BufWriter::new(file);
        Ok(CsvWriter::from_writer(writer))
    }
}

impl<W: Write> CsvWriter<W> {
    /// Create a writer from any output target
    pub fn from_writer(writer: W) -> Self {
        CsvWriter {
            writer,
            config: CsvConfig::default(),
        }
    }

    /// Set the CSV configuration
    ///
    /// # Parameters
    ///
    /// * `config` - CSV configuration (delimiter, quote, escape, etc.)
    pub fn with_config(mut self, config: CsvConfig) -> Self {
        self.config = config;
        self
    }

    /// Write a header row
    ///
    /// # Parameters
    ///
    /// * `headers` - Column header names
    pub fn write_headers(&mut self, headers: &[&str]) -> CsvResult<()> {
        self.write_record_str(headers)
    }

    /// Write a single record (generic version)
    ///
    /// Accepts any type that implements `AsRef<str>`, including `&str`,
    /// `String`, and `Cow<str>`.
    ///
    /// # Parameters
    ///
    /// * `record` - The record fields to write
    pub fn write_record<S: AsRef<str>>(&mut self, record: &[S]) -> CsvResult<()> {
        self.write_record_str(
            &record
                .iter()
                .map(|s| s.as_ref())
                .collect::<Vec<&str>>(),
        )
    }

    /// Write a single [`StringRecord`]
    ///
    /// # Parameters
    ///
    /// * `record` - The record to write
    pub fn write_string_record(&mut self, record: &StringRecord) -> CsvResult<()> {
        self.write_record_str(
            &record
                .iter()
                .map(|s| s.as_str())
                .collect::<Vec<&str>>(),
        )
    }

    /// Write a single record from string slices
    ///
    /// # Parameters
    ///
    /// * `record` - The fields as `&str` values
    pub fn write_record_str(&mut self, record: &[&str]) -> CsvResult<()> {
        let delimiter = self.config.delimiter as char;
        let quote = self.config.quote as char;

        let line: String = record
            .iter()
            .enumerate()
            .map(|(i, field)| {
                if i > 0 {
                    format!("{}{}", delimiter, self.quote_field(field, quote))
                } else {
                    self.quote_field(field, quote)
                }
            })
            .collect();

        writeln!(self.writer, "{}", line)?;
        Ok(())
    }

    /// Write multiple records at once
    ///
    /// # Parameters
    ///
    /// * `records` - The records to write
    pub fn write_all<S: AsRef<str>>(&mut self, records: &[Vec<S>]) -> CsvResult<()> {
        for record in records {
            self.write_record(record)?;
        }
        Ok(())
    }

    /// Flush the underlying writer
    ///
    /// Ensures all buffered data is written to the output target.
    pub fn flush(&mut self) -> CsvResult<()> {
        self.writer.flush()?;
        Ok(())
    }

    fn needs_quoting(field: &str, delimiter: char, quote: char, escape: Option<char>) -> bool {
        field.contains(delimiter)
            || field.contains(quote)
            || field.contains('\n')
            || field.contains('\r')
            || (escape.is_some() && field.contains(escape.unwrap()))
            || field.starts_with(' ')
            || field.ends_with(' ')
    }

    fn quote_field(&self, field: &str, quote: char) -> String {
        let escape = self.config.escape.map(|e| e as char);

        if Self::needs_quoting(field, self.config.delimiter as char, quote, escape) {
            let escaped = if let Some(esc) = escape {
                let mut result = String::with_capacity(field.len() + 4);
                for ch in field.chars() {
                    if ch == quote || ch == esc {
                        result.push(esc);
                    }
                    result.push(ch);
                }
                result
            } else {
                field.replace(quote, &format!("{}{}", quote, quote))
            };
            format!("{}{}{}", quote, escaped, quote)
        } else {
            field.to_string()
        }
    }
}