use crate::config::CsvConfig;
use crate::error::{CsvError, CsvResult};
use crate::parser::CsvParser;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub struct CsvReader<R: BufRead> {
    reader: R,
    parser: CsvParser,
    headers: Option<Vec<String>>,
    current_line: usize,
    byte_offset: u64,
    headers_read: bool,
}

pub type FileReader = CsvReader<BufReader<File>>;

impl FileReader {
    pub fn open<P: AsRef<Path>>(path: P) -> CsvResult<Self> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        Ok(CsvReader::from_reader(reader))
    }
}

impl<R: BufRead> CsvReader<R> {
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

    pub fn with_config(mut self, config: CsvConfig) -> Self {
        self.parser = CsvParser::new(config.clone());
        self
    }

    pub fn headers(&self) -> Option<&[String]> {
        self.headers.as_deref()
    }

    pub fn position(&self) -> (usize, u64) {
        (self.current_line, self.byte_offset)
    }

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

    pub fn read_all(&mut self) -> CsvResult<Vec<Vec<String>>> {
        let mut records: Vec<Vec<String>> = Vec::new();
        for result in self.records() {
            records.push(result?);
        }
        Ok(records)
    }

    pub fn records(&mut self) -> CsvRecordsIter<'_, R> {
        CsvRecordsIter {
            reader: self,
            first_call: true,
            headers_len: None,
        }
    }
}

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

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    fn reader_from_str(s: &str) -> CsvReader<Cursor<&str>> {
        CsvReader::from_reader(Cursor::new(s))
    }

    #[test]
    fn test_basic_read() {
        let mut reader = reader_from_str("a,b,c\nd,e,f\n");
        let records = reader.read_all().unwrap();
        assert_eq!(records.len(), 2);
        assert_eq!(records[0], vec!["a", "b", "c"]);
        assert_eq!(records[1], vec!["d", "e", "f"]);
    }

    #[test]
    fn test_with_headers() {
        let config = CsvConfig::builder().has_headers(true).build();
        let mut reader = reader_from_str("name,age\nAlice,30\nBob,25\n").with_config(config);
        let records = reader.read_all().unwrap();
        assert_eq!(reader.headers().unwrap(), &["name", "age"]);
        assert_eq!(records.len(), 2);
    }

    #[test]
    fn test_flexible_mode() {
        let config = CsvConfig::builder().has_headers(true).flexible(true).build();
        let mut reader = reader_from_str("a,b\n1,2,3\n4,5\n").with_config(config);
        let records = reader.read_all().unwrap();
        assert_eq!(records.len(), 2);
        assert_eq!(records[0].len(), 3);
        assert_eq!(records[1].len(), 2);
    }

    #[test]
    #[should_panic]
    fn test_strict_mode_mismatch() {
        let config = CsvConfig::builder().has_headers(true).build();
        let mut reader = reader_from_str("a,b\n1,2,3\n").with_config(config);
        reader.read_all().unwrap();
    }

    #[test]
    fn test_comment_skip() {
        let config = CsvConfig::builder().comment(b'#').build();
        let mut reader = reader_from_str("# comment\na,b\n# another\nc,d\n").with_config(config);
        let records = reader.read_all().unwrap();
        assert_eq!(records.len(), 2);
        assert_eq!(records[0], vec!["a", "b"]);
        assert_eq!(records[1], vec!["c", "d"]);
    }

    #[test]
    fn test_different_delimiter() {
        let config = CsvConfig::builder().delimiter(b';').build();
        let mut reader = reader_from_str("a;b;c\nd;e;f\n").with_config(config);
        let records = reader.read_all().unwrap();
        assert_eq!(records[0], vec!["a", "b", "c"]);
    }
}