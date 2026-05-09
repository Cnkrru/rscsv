//! String record module
//!
//! Provides a structured container for CSV records with accessor methods.

use std::fmt;
use std::ops::Index;

/// A single CSV record (row) as a collection of field strings
///
/// Supports indexing, iteration, and conversion to/from `Vec<String>`.
///
/// # Examples
///
/// ```rust
/// use rscsv::StringRecord;
///
/// let record = StringRecord::new(vec!["Alice".into(), "30".into()]);
/// assert_eq!(record.get(0), Some("Alice"));
/// assert_eq!(record[1], "30");
/// assert_eq!(record.len(), 2);
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StringRecord {
    fields: Vec<String>,
}

impl StringRecord {
    /// Create a new record from a vector of fields
    pub fn new(fields: Vec<String>) -> Self {
        StringRecord { fields }
    }

    /// Returns the number of fields in the record
    pub fn len(&self) -> usize {
        self.fields.len()
    }

    /// Returns true if the record has no fields
    pub fn is_empty(&self) -> bool {
        self.fields.is_empty()
    }

    /// Returns the number of fields (alias for `len`)
    pub fn field_count(&self) -> usize {
        self.fields.len()
    }

    /// Access a field by index, returning `None` if out of bounds
    pub fn get(&self, index: usize) -> Option<&str> {
        self.fields.get(index).map(|s| s.as_str())
    }

    /// Returns the first field, or `None` if empty
    pub fn first(&self) -> Option<&str> {
        self.fields.first().map(|s| s.as_str())
    }

    /// Returns the last field, or `None` if empty
    pub fn last(&self) -> Option<&str> {
        self.fields.last().map(|s| s.as_str())
    }

    /// Returns a slice view of all fields
    pub fn as_slice(&self) -> &[String] {
        &self.fields
    }

    /// Returns an iterator over references to the fields
    pub fn iter(&self) -> impl Iterator<Item = &String> {
        self.fields.iter()
    }

    /// Consumes the record and returns the underlying vector
    pub fn into_vec(self) -> Vec<String> {
        self.fields
    }
}

impl Index<usize> for StringRecord {
    type Output = String;

    /// Access a field by index
    ///
    /// # Panics
    ///
    /// Panics if the index is out of bounds.
    fn index(&self, index: usize) -> &Self::Output {
        &self.fields[index]
    }
}

impl From<Vec<String>> for StringRecord {
    /// Create a record from a vector of strings
    fn from(fields: Vec<String>) -> Self {
        StringRecord { fields }
    }
}

impl IntoIterator for StringRecord {
    type Item = String;
    type IntoIter = std::vec::IntoIter<String>;

    /// Iterate over the fields, consuming the record
    fn into_iter(self) -> Self::IntoIter {
        self.fields.into_iter()
    }
}

impl fmt::Display for StringRecord {
    /// Display the record as comma-separated values
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.fields.join(","))
    }
}