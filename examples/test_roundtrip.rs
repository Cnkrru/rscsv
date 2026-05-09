use rscsv::{CsvConfig, CsvReader, CsvWriter};
use std::io::Cursor;

fn test_roundtrip_simple() {
    let data = vec![
        vec!["name".to_string(), "age".to_string()],
        vec!["Alice".to_string(), "30".to_string()],
        vec!["Bob".to_string(), "25".to_string()],
    ];

    let mut buf = Vec::new();
    {
        let mut writer = CsvWriter::from_writer(Cursor::new(&mut buf));
        writer.write_all(&data).unwrap();
        writer.flush().unwrap();
    }

    let cursor = Cursor::new(buf);
    let mut reader = CsvReader::from_reader(cursor);
    let records = reader.read_all().unwrap();

    assert_eq!(records, data);
    println!("  ✓ test_roundtrip_simple");
}

fn test_roundtrip_with_quoting() {
    let data = vec![
        vec!["a,b".to_string(), "c\"d".to_string()],
    ];

    let mut buf = Vec::new();
    {
        let mut writer = CsvWriter::from_writer(Cursor::new(&mut buf));
        writer.write_all(&data).unwrap();
        writer.flush().unwrap();
    }

    let cursor = Cursor::new(buf);
    let mut reader = CsvReader::from_reader(cursor);
    let records = reader.read_all().unwrap();

    assert_eq!(records, data);
    println!("  ✓ test_roundtrip_with_quoting");
}

fn test_roundtrip_with_headers() {
    let headers = vec!["姓名".to_string(), "年龄".to_string(), "城市".to_string()];
    let records = vec![
        vec!["张三".to_string(), "25".to_string(), "北京".to_string()],
        vec!["李四".to_string(), "30".to_string(), "上海".to_string()],
    ];

    let mut buf = Vec::new();
    {
        let mut writer = CsvWriter::from_writer(Cursor::new(&mut buf));
        writer.write_record(&headers).unwrap();
        writer.write_all(&records).unwrap();
        writer.flush().unwrap();
    }

    let cursor = Cursor::new(buf);
    let mut reader = CsvReader::from_reader(cursor);
    reader.read_headers().unwrap();
    assert_eq!(reader.headers().unwrap(), &headers);

    let read_records = reader.read_all().unwrap();
    assert_eq!(read_records, records);
    println!("  ✓ test_roundtrip_with_headers");
}

fn test_roundtrip_semicolon() {
    let config = CsvConfig::builder().delimiter(b';').build();
    let data = vec![
        vec!["a".to_string(), "b".to_string(), "c".to_string()],
    ];

    let mut buf = Vec::new();
    {
        let mut writer = CsvWriter::from_writer(Cursor::new(&mut buf)).with_config(config.clone());
        writer.write_all(&data).unwrap();
        writer.flush().unwrap();
    }

    let cursor = Cursor::new(buf);
    let mut reader = CsvReader::from_reader(cursor).with_config(config);
    let records = reader.read_all().unwrap();

    assert_eq!(records, data);
    println!("  ✓ test_roundtrip_semicolon");
}

fn main() {
    println!("=== test_roundtrip ===\n");
    test_roundtrip_simple();
    test_roundtrip_with_quoting();
    test_roundtrip_with_headers();
    test_roundtrip_semicolon();
    println!("\n✅ 4 tests passed");
}