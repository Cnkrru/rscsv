//! Configuration module
//!
//! Provides CSV configuration options and builder pattern.

/// CSV configuration structure
///
/// Controls how CSV data is parsed and written, including delimiter choice,
/// quoting behavior, header handling, and comment line support.
///
/// Use [`CsvConfigBuilder`] for convenient construction.
///
/// # Examples
///
/// ```rust
/// use rscsv::CsvConfig;
///
/// let config = CsvConfig::builder()
///     .delimiter(b',')
///     .has_headers(true)
///     .build();
/// ```
#[derive(Clone)]
pub struct CsvConfig {
    /// Field delimiter character (default: `,`)
    pub delimiter: u8,
    /// Quote character for wrapping fields (default: `"`)
    pub quote: u8,
    /// Escape character for escaping special chars inside quoted fields (default: `None`)
    pub escape: Option<u8>,
    /// Whether the first row contains headers (default: `false`)
    pub has_headers: bool,
    /// Allow rows with varying column counts (default: `false`)
    pub flexible: bool,
    /// Trim whitespace from unquoted fields (default: `false`)
    pub trim: bool,
    /// Character that identifies comment lines to skip (default: `None`)
    pub comment: Option<u8>,
}

impl Default for CsvConfig {
    /// Returns the default CSV configuration
    ///
    /// ```rust
    /// use rscsv::CsvConfig;
    ///
    /// let config = CsvConfig::default();
    /// assert_eq!(config.delimiter, b',');
    /// assert_eq!(config.quote, b'"');
    /// assert!(!config.has_headers);
    /// ```
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

impl CsvConfig {
    /// Create a new configuration builder
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rscsv::CsvConfig;
    ///
    /// let config = CsvConfig::builder()
    ///     .delimiter(b';')
    ///     .has_headers(true)
    ///     .build();
    /// ```
    pub fn builder() -> CsvConfigBuilder {
        CsvConfigBuilder::new()
    }
}

/// CSV configuration builder
///
/// Provides a fluent interface for building [`CsvConfig`].
///
/// # Examples
///
/// ```rust
/// use rscsv::CsvConfigBuilder;
///
/// let config = CsvConfigBuilder::new()
///     .delimiter(b'\t')
///     .has_headers(true)
///     .flexible(true)
///     .build();
/// ```
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

    /// Set the field delimiter character
    ///
    /// # Parameters
    ///
    /// * `d` - Delimiter byte (e.g., `b','`, `b';'`, `b'\t'`)
    pub fn delimiter(mut self, d: u8) -> Self {
        self.delimiter = d;
        self
    }

    /// Set the quote character
    ///
    /// # Parameters
    ///
    /// * `q` - Quote byte (e.g., `b'"'`, `b'\''`)
    pub fn quote(mut self, q: u8) -> Self {
        self.quote = q;
        self
    }

    /// Set the escape character for quoted fields
    ///
    /// When set, backslash-style escaping is used inside quoted fields.
    ///
    /// # Parameters
    ///
    /// * `ch` - Escape byte (e.g., `b'\\'`)
    pub fn escape(mut self, ch: u8) -> Self {
        self.escape = Some(ch);
        self
    }

    /// Set whether the first row contains headers
    ///
    /// When enabled, the first row is automatically read as headers.
    ///
    /// # Parameters
    ///
    /// * `yes` - `true` to enable header detection
    pub fn has_headers(mut self, yes: bool) -> Self {
        self.has_headers = yes;
        self
    }

    /// Set flexible mode (allow varying column counts)
    ///
    /// When enabled, rows with different column counts are accepted.
    /// When disabled, mismatched column counts produce errors.
    ///
    /// # Parameters
    ///
    /// * `yes` - `true` to enable flexible mode
    pub fn flexible(mut self, yes: bool) -> Self {
        self.flexible = yes;
        self
    }

    /// Set whether to trim whitespace from unquoted fields
    ///
    /// # Parameters
    ///
    /// * `yes` - `true` to trim leading/trailing whitespace
    pub fn trim(mut self, yes: bool) -> Self {
        self.trim = yes;
        self
    }

    /// Set the comment character
    ///
    /// Lines starting with this character are skipped during parsing.
    ///
    /// # Parameters
    ///
    /// * `ch` - Comment byte (e.g., `b'#'`)
    pub fn comment(mut self, ch: u8) -> Self {
        self.comment = Some(ch);
        self
    }

    /// Build the final configuration
    ///
    /// # Returns
    ///
    /// A [`CsvConfig`] instance with all configured values.
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