use rscsv::{CsvConfig, CsvParser};

fn test_simple_line() {
    let parser = CsvParser::new(CsvConfig::default());
    let result = parser.parse_line("a,b,c", 1).unwrap();
    assert_eq!(result, vec!["a", "b", "c"]);
    println!("  ✓ test_simple_line");
}

fn test_quoted_field() {
    let parser = CsvParser::new(CsvConfig::default());
    let result = parser.parse_line(r#""hello, world",b,c"#, 1).unwrap();
    assert_eq!(result, vec!["hello, world", "b", "c"]);
    println!("  ✓ test_quoted_field");
}

fn test_escaped_quote() {
    let parser = CsvParser::new(CsvConfig::default());
    let result = parser.parse_line(r#""say ""hello""",b"#, 1).unwrap();
    assert_eq!(result, vec![r#"say "hello""#, "b"]);
    println!("  ✓ test_escaped_quote");
}

fn test_empty_fields() {
    let parser = CsvParser::new(CsvConfig::default());
    let result = parser.parse_line("a,,c", 1).unwrap();
    assert_eq!(result, vec!["a", "", "c"]);
    println!("  ✓ test_empty_fields");
}

fn test_quoted_newline() {
    let parser = CsvParser::new(CsvConfig::default());
    let result = parser.parse_line("\"a\nb\",c", 1).unwrap();
    assert_eq!(result, vec!["a\nb", "c"]);
    println!("  ✓ test_quoted_newline");
}

fn test_custom_delimiter() {
    let config = CsvConfig::builder().delimiter(b';').build();
    let parser = CsvParser::new(config);
    let result = parser.parse_line("a;b;c", 1).unwrap();
    assert_eq!(result, vec!["a", "b", "c"]);
    println!("  ✓ test_custom_delimiter");
}

fn test_trim() {
    let config = CsvConfig::builder().trim(true).build();
    let parser = CsvParser::new(config);
    let result = parser.parse_line(" a , b , c ", 1).unwrap();
    assert_eq!(result, vec!["a", "b", "c"]);
    println!("  ✓ test_trim");
}

fn main() {
    println!("=== test_parser ===\n");
    test_simple_line();
    test_quoted_field();
    test_escaped_quote();
    test_empty_fields();
    test_quoted_newline();
    test_custom_delimiter();
    test_trim();
    println!("\n✅ 7 tests passed");
}