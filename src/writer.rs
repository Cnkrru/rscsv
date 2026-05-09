use crate::config::CsvConfig;
use crate::error::CsvResult;
use crate::record::StringRecord;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

pub struct CsvWriter<W: Write> {
    writer: W,
    config: CsvConfig,
}

pub type FileWriter = CsvWriter<BufWriter<File>>;

impl FileWriter {
    pub fn create<P: AsRef<Path>>(path: P) -> CsvResult<Self> {
        let file = File::create(path)?;
        let writer = BufWriter::new(file);
        Ok(CsvWriter::from_writer(writer))
    }
}

impl<W: Write> CsvWriter<W> {
    pub fn from_writer(writer: W) -> Self {
        CsvWriter {
            writer,
            config: CsvConfig::default(),
        }
    }

    pub fn with_config(mut self, config: CsvConfig) -> Self {
        self.config = config;
        self
    }

    pub fn write_headers(&mut self, headers: &[&str]) -> CsvResult<()> {
        self.write_record_str(headers)
    }

    pub fn write_record<S: AsRef<str>>(&mut self, record: &[S]) -> CsvResult<()> {
        self.write_record_str(
            &record
                .iter()
                .map(|s| s.as_ref())
                .collect::<Vec<&str>>(),
        )
    }

    pub fn write_string_record(&mut self, record: &StringRecord) -> CsvResult<()> {
        self.write_record_str(
            &record
                .iter()
                .map(|s| s.as_str())
                .collect::<Vec<&str>>(),
        )
    }

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

    pub fn write_all<S: AsRef<str>>(&mut self, records: &[Vec<S>]) -> CsvResult<()> {
        for record in records {
            self.write_record(record)?;
        }
        Ok(())
    }

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::reader::CsvReader;
    use std::io::Cursor;

    #[test]
    fn test_basic_write() {
        let mut buf = Vec::new();
        {
            let mut writer = CsvWriter::from_writer(&mut buf);
            writer.write_record_str(&["a", "b", "c"]).unwrap();
        }
        let result = String::from_utf8(buf).unwrap();
        assert_eq!(result, "a,b,c\n");
    }

    #[test]
    fn test_write_quoted_field() {
        let mut buf = Vec::new();
        {
            let mut writer = CsvWriter::from_writer(&mut buf);
            writer.write_record_str(&["hello, world", "b"]).unwrap();
        }
        let result = String::from_utf8(buf).unwrap();
        assert_eq!(result, "\"hello, world\",b\n");
    }

    #[test]
    fn test_write_with_escape_char() {
        let mut buf = Vec::new();
        {
            let config = CsvConfig::builder().escape(b'\\').build();
            let mut writer = CsvWriter::from_writer(&mut buf).with_config(config);
            writer.write_record_str(&[r#"say "hello""#, "b"]).unwrap();
        }
        let result = String::from_utf8(buf).unwrap();
        let expected = "\"say \\\"hello\\\"\",b\n";
        assert_eq!(result, expected);
    }

    #[test]
    fn test_generic_write_record() {
        let mut buf = Vec::new();
        {
            let mut writer = CsvWriter::from_writer(&mut buf);
            writer.write_record(&["a".to_string(), "b".to_string()]).unwrap();
            writer.write_record(&["c", "d"]).unwrap();
        }
        let result = String::from_utf8(buf).unwrap();
        assert_eq!(result, "a,b\nc,d\n");
    }

    #[test]
    fn test_write_string_record() {
        let mut buf = Vec::new();
        {
            let record = StringRecord::new(vec!["x".to_string(), "y".to_string()]);
            let mut writer = CsvWriter::from_writer(&mut buf);
            writer.write_string_record(&record).unwrap();
        }
        let result = String::from_utf8(buf).unwrap();
        assert_eq!(result, "x,y\n");
    }

    #[test]
    fn test_roundtrip() {
        let mut buf = Vec::new();
        let data = vec![vec!["a".to_string(), "b".to_string()], vec!["c".to_string(), "d".to_string()]];
        {
            let mut writer = CsvWriter::from_writer(&mut buf);
            writer.write_all(&data).unwrap();
        }
        let mut reader = CsvReader::from_reader(Cursor::new(&buf));
        let records = reader.read_all().unwrap();
        assert_eq!(records, data);
    }
}