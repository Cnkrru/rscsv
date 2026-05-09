//! CSV reader module
//!
//! Provides a buffered CSV reader with support for headers, comments,
//! flexible column counts, and multi-line quoted fields.

use crate::config::CsvConfig;
use crate::error::{CsvError, CsvResult};
use crate::parser::CsvParser;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

/// CSV reader
///
/// Reads CSV data from any [`BufRead`] source, supporting headers,
/// comment lines, flexible column counts, and multi-line quoted fields.
///
/// # Examples
///
/// ```rust
/// use rscsv::CsvReader;
///
/// let data = "a,b,c\nd,e,f\n";
/// let mut reader = CsvReader::from_reader(data.as_bytes());
/// let records = reader.read_all().unwrap();
/// assert_eq!(records.len(), 2);
/// ```
pub struct CsvReader<R: BufRead> {
    reader: R,
    parser: CsvParser,
    headers: Option<Vec<String>>,
    current_line: usize,
    byte_offset: u64,
    headers_read: bool,
}

/// Convenience type alias for reading CSV from a file
pub type FileReader = CsvReader<BufReader<File>>;

impl FileReader {
    /// Open a CSV file for reading
    ///
    /// # Parameters
    ///
    /// * `path` - Path to the CSV file
    ///
    /// # Returns
    ///
    /// A new [`FileReader`] instance
    pub fn open<P: AsRef<Path>>(path: P) -> CsvResult<Self> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        Ok(CsvReader::from_reader(reader))
    }
}

impl<R: BufRead> CsvReader<R> {
    /// Create a reader from any buffered input source
    ///
    /// Uses default configuration. Use [`with_config`](CsvReader::with_config)
    /// to customize.
    pub fn from_reader(reader: R) -> Self {
        CsvReader {
            reader,
            parser: CsvParser::new(CsvConfig::default()),
            headers: None,
            current_line: 0,
            byte_offset: 0,
            headers_read: false,
        }
    }

    /// Set the CSV configuration
    ///
    /// # Parameters
    ///
    /// * `config` - CSV configuration
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rscsv::{CsvReader, CsvConfig};
    ///
    /// let config = CsvConfig::builder()
    ///     .delimiter(b';')
    ///     .has_headers(true)
    ///     .build();
    ///
    /// let mut reader = CsvReader::from_reader("a;b\n1;2\n".as_bytes())
    ///     .with_config(config);
    /// ```
    pub fn with_config(mut self, config: CsvConfig) -> Self {
        self.parser = CsvParser::new(config.clone());
        self
    }

    /// Returns the headers if they have been read
    ///
    /// Headers are read automatically when `has_headers` is enabled.
    pub fn headers(&self) -> Option<&[String]> {
        self.headers.as_deref()
    }

    /// Returns the current reading position
    ///
    /// # Returns
    ///
    /// A tuple of `(line_number, byte_offset)`
    pub fn position(&self) -> (usize, u64) {
        (self.current_line, self.byte_offset)
    }

    /// Explicitly read the header row
    ///
    /// Usually called automatically when `has_headers` is enabled.
    ///
    /// # Errors
    ///
    /// Returns an error if the file is empty.
    pub fn read_headers(&mut self) -> CsvResult<()> {
        let mut line = String::new();
        let bytes = self.reader.read_line(&mut line)?;
        if bytes == 0 {
            return Err(CsvError::Parse {
                line: 1,
                col: 1,
                message: "文件为空，无法读取表头".to_string(),
            });
        }
        self.current_line = 1;
        self.byte_offset += bytes as u64;
        self.headers_read = true;
        self.headers = Some(self.parser.parse_header(line.trim_end_matches(['\r', '\n']))?);
        Ok(())
    }

    fn ensure_headers(&mut self) -> CsvResult<()> {
        if self.parser.config.has_headers && !self.headers_read {
            self.read_headers()?;
        }
        Ok(())
    }

    /// Read all remaining records into a vector
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rscsv::CsvReader;
    ///
    /// let mut reader = CsvReader::from_reader("a,b\nc,d\n".as_bytes());
    /// let records = reader.read_all().unwrap();
    /// assert_eq!(records, vec![
    ///     vec!["a".to_string(), "b".to_string()],
    ///     vec!["c".to_string(), "d".to_string()],
    /// ]);
    /// ```
    pub fn read_all(&mut self) -> CsvResult<Vec<Vec<String>>> {
        let mut records: Vec<Vec<String>> = Vec::new();
        for result in self.records() {
            records.push(result?);
        }
        Ok(records)
    }

    /// Returns an iterator over the remaining records
    ///
    /// Handles multi-line quoted fields, comment skipping, and
    /// column count validation automatically.
    pub fn records(&mut self) -> CsvRecordsIter<'_, R> {
        CsvRecordsIter {
            reader: self,
            first_call: true,
            headers_len: None,
        }
    }
}

/// Iterator over CSV records
///
/// Created by [`CsvReader::records()`]. Handles multi-line quoted fields,
/// comment lines, and flexible/strict column validation.
pub struct CsvRecordsIter<'a, R: BufRead> {
    reader: &'a mut CsvReader<R>,
    first_call: bool,
    headers_len: Option<usize>,
}

impl<'a, R: BufRead> Iterator for CsvRecordsIter<'a, R> {
    type Item = CsvResult<Vec<String>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.first_call {
            self.first_call = false;
            if let Err(e) = self.reader.ensure_headers() {
                return Some(Err(e));
            }
            self.headers_len = self.reader.headers.as_ref().map(|h| h.len());
        }

        let config = self.reader.parser.config.clone();
        let comment = config.comment.map(|c| c as char);
        let flexible = config.flexible;
        let headers_len = self.headers_len;

        let mut line = String::new();
        let mut raw = String::new();
        let mut in_quotes = false;
        let quote = config.quote as char;

        loop {
            line.clear();
            match self.reader.reader.read_line(&mut line) {
                Ok(0) => {
                    if raw.is_empty() {
                        return None;
                    }
                    self.reader.current_line += 1;
                    return Some(self.finish_record(&raw, flexible, headers_len));
                }
                Ok(bytes) => {
                    raw.push_str(&line);
                    self.reader.byte_offset += bytes as u64;
                    let chars: Vec<char> = line.chars().collect();
                    for &ch in &chars {
                        if ch == quote {
                            in_quotes = !in_quotes;
                        }
                    }
                    if !in_quotes {
                        self.reader.current_line += 1;
                        let trimmed = raw.trim_end_matches(&['\r', '\n'][..]).to_string();

                        if let Some(comm) = comment {
                            if trimmed.starts_with(comm) {
                                raw.clear();
                                continue;
                            }
                        }

                        let result = self.finish_record(&trimmed, flexible, headers_len);
                        raw.clear();
                        return Some(result);
                    }
                }
                Err(e) => return Some(Err(CsvError::Io(e))),
            }
        }
    }
}

impl<'a, R: BufRead> CsvRecordsIter<'a, R> {
    fn finish_record(
        &self,
        raw: &str,
        flexible: bool,
        headers_len: Option<usize>,
    ) -> CsvResult<Vec<String>> {
        let fields = self
            .reader
            .parser
            .parse_line(raw, self.reader.current_line)?;

        if !flexible {
            if let Some(expected) = headers_len {
                if fields.len() != expected {
                    return Err(CsvError::Parse {
                        line: self.reader.current_line,
                        col: 1,
                        message: format!(
                            "字段数不匹配: 期望 {} 个字段，实际 {} 个字段",
                            expected,
                            fields.len()
                        ),
                    });
                }
            }
        }

        Ok(fields)
    }
}