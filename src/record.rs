use std::fmt;
use std::ops::Index;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StringRecord {
    fields: Vec<String>,
}

impl StringRecord {
    pub fn new(fields: Vec<String>) -> Self {
        StringRecord { fields }
    }

    pub fn len(&self) -> usize {
        self.fields.len()
    }

    pub fn is_empty(&self) -> bool {
        self.fields.is_empty()
    }

    pub fn field_count(&self) -> usize {
        self.fields.len()
    }

    pub fn get(&self, index: usize) -> Option<&str> {
        self.fields.get(index).map(|s| s.as_str())
    }

    pub fn first(&self) -> Option<&str> {
        self.fields.first().map(|s| s.as_str())
    }

    pub fn last(&self) -> Option<&str> {
        self.fields.last().map(|s| s.as_str())
    }

    pub fn as_slice(&self) -> &[String] {
        &self.fields
    }

    pub fn iter(&self) -> impl Iterator<Item = &String> {
        self.fields.iter()
    }

    pub fn into_vec(self) -> Vec<String> {
        self.fields
    }
}

impl Index<usize> for StringRecord {
    type Output = String;

    fn index(&self, index: usize) -> &Self::Output {
        &self.fields[index]
    }
}

impl From<Vec<String>> for StringRecord {
    fn from(fields: Vec<String>) -> Self {
        StringRecord { fields }
    }
}

impl IntoIterator for StringRecord {
    type Item = String;
    type IntoIter = std::vec::IntoIter<String>;

    fn into_iter(self) -> Self::IntoIter {
        self.fields.into_iter()
    }
}

impl fmt::Display for StringRecord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.fields.join(","))
    }
}