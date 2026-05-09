use rscsv::CsvReader;
use std::io::Cursor;

fn test_read_from_string() {
    let data = "a,b,c\n1,2,3\n4,5,6\n";
    let cursor = Cursor::new(data);
    let mut reader = CsvReader::from_reader(cursor);
    let records = reader.read_all().unwrap();
    assert_eq!(records.len(), 3);
    assert_eq!(records[0], vec!["a", "b", "c"]);
    assert_eq!(records[1], vec!["1", "2", "3"]);
    assert_eq!(records[2], vec!["4", "5", "6"]);
    println!("  ✓ test_read_from_string");
}

fn test_read_with_headers() {
    let data = "name,age,city\nAlice,30,Beijing\n";
    let cursor = Cursor::new(data);
    let mut reader = CsvReader::from_reader(cursor);
    reader.read_headers().unwrap();
    assert_eq!(reader.headers().unwrap(), &vec!["name", "age", "city"]);
    let records = reader.read_all().unwrap();
    assert_eq!(records[0], vec!["Alice", "30", "Beijing"]);
    println!("  ✓ test_read_with_headers");
}

fn test_quoted_fields_in_stream() {
    let data = "\"hello, world\",\"say \"\"hi\"\"\"\nvalue1,value2\n";
    let cursor = Cursor::new(data);
    let mut reader = CsvReader::from_reader(cursor);
    let records = reader.read_all().unwrap();
    assert_eq!(records[0], vec!["hello, world", "say \"hi\""]);
    assert_eq!(records[1], vec!["value1", "value2"]);
    println!("  ✓ test_quoted_fields_in_stream");
}

fn main() {
    println!("=== test_reader ===\n");
    test_read_from_string();
    test_read_with_headers();
    test_quoted_fields_in_stream();
    println!("\n✅ 3 tests passed");
}