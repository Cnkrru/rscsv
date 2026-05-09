#[derive(Clone)]
pub struct CsvConfig {
    pub delimiter: u8,
    pub quote: u8,
    pub escape: Option<u8>,
    pub has_headers: bool,
    pub flexible: bool,
    pub trim: bool,
    pub comment: Option<u8>,
}

impl Default for CsvConfig {
    fn default() -> Self {
        CsvConfig {
            delimiter: b',',
            quote: b'"',
            escape: None,
            has_headers: false,
            flexible: false,
            trim: false,
            comment: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_values() {
        let config = CsvConfig::default();
        assert_eq!(config.delimiter, b',');
        assert_eq!(config.quote, b'"');
        assert_eq!(config.escape, None);
        assert!(!config.has_headers);
        assert!(!config.flexible);
        assert!(!config.trim);
        assert_eq!(config.comment, None);
    }

    #[test]
    fn test_builder_custom() {
        let config = CsvConfig::builder()
            .delimiter(b';')
            .quote(b'\'')
            .escape(b'\\')
            .has_headers(true)
            .flexible(true)
            .trim(true)
            .comment(b'#')
            .build();

        assert_eq!(config.delimiter, b';');
        assert_eq!(config.quote, b'\'');
        assert_eq!(config.escape, Some(b'\\'));
        assert!(config.has_headers);
        assert!(config.flexible);
        assert!(config.trim);
        assert_eq!(config.comment, Some(b'#'));
    }

    #[test]
    fn test_builder_partial() {
        let config = CsvConfig::builder()
            .delimiter(b'\t')
            .has_headers(true)
            .build();

        assert_eq!(config.delimiter, b'\t');
        assert_eq!(config.quote, b'"');
        assert_eq!(config.escape, None);
        assert!(config.has_headers);
        assert!(!config.flexible);
    }
}

impl CsvConfig {
    pub fn builder() -> CsvConfigBuilder {
        CsvConfigBuilder::new()
    }
}

pub struct CsvConfigBuilder {
    delimiter: u8,
    quote: u8,
    escape: Option<u8>,
    has_headers: bool,
    flexible: bool,
    trim: bool,
    comment: Option<u8>,
}

impl CsvConfigBuilder {
    fn new() -> Self {
        CsvConfigBuilder {
            delimiter: b',',
            quote: b'"',
            escape: None,
            has_headers: false,
            flexible: false,
            trim: false,
            comment: None,
        }
    }

    pub fn delimiter(mut self, d: u8) -> Self {
        self.delimiter = d;
        self
    }

    pub fn quote(mut self, q: u8) -> Self {
        self.quote = q;
        self
    }

    pub fn escape(mut self, ch: u8) -> Self {
        self.escape = Some(ch);
        self
    }

    pub fn has_headers(mut self, yes: bool) -> Self {
        self.has_headers = yes;
        self
    }

    pub fn flexible(mut self, yes: bool) -> Self {
        self.flexible = yes;
        self
    }

    pub fn trim(mut self, yes: bool) -> Self {
        self.trim = yes;
        self
    }

    pub fn comment(mut self, ch: u8) -> Self {
        self.comment = Some(ch);
        self
    }

    pub fn build(self) -> CsvConfig {
        CsvConfig {
            delimiter: self.delimiter,
            quote: self.quote,
            escape: self.escape,
            has_headers: self.has_headers,
            flexible: self.flexible,
            trim: self.trim,
            comment: self.comment,
        }
    }
}