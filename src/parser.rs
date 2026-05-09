use crate::config::CsvConfig;
use crate::error::{CsvError, CsvResult};

pub struct CsvParser {
    pub(crate) config: CsvConfig,
}

struct CsvParserState {
    line: usize,
    col: usize,
    chars: Vec<char>,
    pos: usize,
}

impl CsvParser {
    pub fn new(config: CsvConfig) -> Self {
        CsvParser { config }
    }

    pub fn parse_line(&self, raw_line: &str, line_num: usize) -> CsvResult<Vec<String>> {
        let state = CsvParserState {
            line: line_num,
            col: 1,
            chars: raw_line.chars().collect(),
            pos: 0,
        };
        self.parse_fields(state)
    }

    pub fn parse_header(&self, raw_line: &str) -> CsvResult<Vec<String>> {
        self.parse_line(raw_line, 1)
    }

    fn parse_fields(&self, mut state: CsvParserState) -> CsvResult<Vec<String>> {
        let mut fields: Vec<String> = Vec::new();
        let delimiter = self.config.delimiter as char;
        let quote = self.config.quote as char;

        while state.pos < state.chars.len() {
            let field = self.parse_field(&mut state, delimiter, quote)?;
            fields.push(field);

            if state.pos < state.chars.len() {
                let ch = state.chars[state.pos];
                if ch == delimiter {
                    state.pos += 1;
                    state.col += 1;
                    if state.pos >= state.chars.len() {
                        fields.push(String::new());
                    }
                } else if ch == '\n' {
                    break;
                } else if ch == '\r' {
                    state.pos += 1;
                    if state.pos < state.chars.len() && state.chars[state.pos] == '\n' {
                        state.pos += 1;
                    }
                    break;
                }
            }
        }

        Ok(fields)
    }

    fn parse_field(
        &self,
        state: &mut CsvParserState,
        delimiter: char,
        quote: char,
    ) -> CsvResult<String> {
        if state.pos >= state.chars.len() {
            return Ok(String::new());
        }

        let ch = state.chars[state.pos];

        if ch == quote {
            self.parse_quoted_field(state, quote)
        } else {
            self.parse_unquoted_field(state, delimiter)
        }
    }

    fn parse_quoted_field(&self, state: &mut CsvParserState, quote: char) -> CsvResult<String> {
        state.pos += 1;
        let mut field = String::new();
        let mut last_was_quote = false;
        let escape_char = self.config.escape.map(|e| e as char);

        while state.pos < state.chars.len() {
            let ch = state.chars[state.pos];

            if let Some(esc) = escape_char {
                if ch == esc {
                    state.pos += 1;
                    if state.pos < state.chars.len() {
                        let next = state.chars[state.pos];
                        field.push(next);
                        state.pos += 1;
                    } else {
                        return Err(CsvError::InvalidEscape {
                            line: state.line,
                            col: state.col,
                        });
                    }
                    continue;
                }
            }

            if last_was_quote {
                last_was_quote = false;
                if ch == quote {
                    field.push(quote);
                    state.pos += 1;
                    continue;
                } else {
                    break;
                }
            }

            if ch == quote {
                last_was_quote = true;
                state.pos += 1;
                continue;
            }

            if ch == '\n' || ch == '\r' {
                break;
            }

            field.push(ch);
            state.pos += 1;
        }

        Ok(field)
    }

    fn parse_unquoted_field(
        &self,
        state: &mut CsvParserState,
        delimiter: char,
    ) -> CsvResult<String> {
        let start = state.pos;
        let mut end = state.pos;

        while state.pos < state.chars.len() {
            let ch = state.chars[state.pos];
            if ch == delimiter || ch == '\n' || ch == '\r' {
                break;
            }
            end = state.pos + 1;
            state.pos += 1;
        }

        let mut field: String = state.chars[start..end].iter().collect();

        if self.config.trim {
            field = field.trim().to_string();
        }

        Ok(field)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::CsvConfig;

    #[test]
    fn test_basic_fields() {
        let parser = CsvParser::new(CsvConfig::default());
        let result = parser.parse_line("a,b,c", 1).unwrap();
        assert_eq!(result, vec!["a", "b", "c"]);
    }

    #[test]
    fn test_quoted_field() {
        let parser = CsvParser::new(CsvConfig::default());
        let result = parser.parse_line(r#""hello, world",b,c"#, 1).unwrap();
        assert_eq!(result, vec!["hello, world", "b", "c"]);
    }

    #[test]
    fn test_escaped_quotes() {
        let parser = CsvParser::new(CsvConfig::default());
        let result = parser.parse_line(r#""he said ""hi""",b"#, 1).unwrap();
        assert_eq!(result, vec![r#"he said "hi""#, "b"]);
    }

    #[test]
    fn test_escape_char() {
        let config = CsvConfig::builder().escape(b'\\').build();
        let parser = CsvParser::new(config);
        let result = parser.parse_line(r#""hello \"world\"",b"#, 1).unwrap();
        assert_eq!(result, vec![r#"hello "world""#, "b"]);
    }

    #[test]
    fn test_empty_fields() {
        let parser = CsvParser::new(CsvConfig::default());
        let result = parser.parse_line("a,,c", 1).unwrap();
        assert_eq!(result, vec!["a", "", "c"]);
    }

    #[test]
    fn test_delimiter_only() {
        let parser = CsvParser::new(CsvConfig::default());
        let result = parser.parse_line(",", 1).unwrap();
        assert_eq!(result, vec!["", ""]);
    }

    #[test]
    fn test_trim() {
        let config = CsvConfig::builder().trim(true).build();
        let parser = CsvParser::new(config);
        let result = parser.parse_line(" a , b , c ", 1).unwrap();
        assert_eq!(result, vec!["a", "b", "c"]);
    }

    #[test]
    fn test_custom_delimiter() {
        let config = CsvConfig::builder().delimiter(b';').build();
        let parser = CsvParser::new(config);
        let result = parser.parse_line("a;b;c", 1).unwrap();
        assert_eq!(result, vec!["a", "b", "c"]);
    }
}