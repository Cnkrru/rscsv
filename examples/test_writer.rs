use rscsv::{CsvConfig, CsvWriter};
use std::io::Cursor;

fn test_write_simple() {
    let mut buf = Vec::new();
    {
        let mut writer = CsvWriter::from_writer(Cursor::new(&mut buf));
        writer
            .write_all(&[
                vec!["a".to_string(), "b".to_string(), "c".to_string()],
                vec!["1".to_string(), "2".to_string(), "3".to_string()],
            ])
            .unwrap();
        writer.flush().unwrap();
    }
    let output = String::from_utf8(buf).unwrap();
    assert_eq!(output, "a,b,c\n1,2,3\n");
    println!("  ✓ test_write_simple");
}

fn test_write_with_quoting() {
    let mut buf = Vec::new();
    {
        let mut writer = CsvWriter::from_writer(Cursor::new(&mut buf));
        writer
            .write_record(&vec!["hello, world".to_string(), "say \"hi\"".to_string()])
            .unwrap();
        writer.flush().unwrap();
    }
    let output = String::from_utf8(buf).unwrap();
    assert_eq!(output, "\"hello, world\",\"say \"\"hi\"\"\"\n");
    println!("  ✓ test_write_with_quoting");
}

fn test_write_with_custom_delimiter() {
    let config = CsvConfig::builder().delimiter(b';').build();
    let mut buf = Vec::new();
    {
        let mut writer = CsvWriter::from_writer(Cursor::new(&mut buf)).with_config(config);
        writer
            .write_record(&vec!["a".to_string(), "b".to_string(), "c".to_string()])
            .unwrap();
        writer.flush().unwrap();
    }
    let output = String::from_utf8(buf).unwrap();
    assert_eq!(output, "a;b;c\n");
    println!("  ✓ test_write_with_custom_delimiter");
}

fn main() {
    println!("=== test_writer ===\n");
    test_write_simple();
    test_write_with_quoting();
    test_write_with_custom_delimiter();
    println!("\n✅ 3 tests passed");
}