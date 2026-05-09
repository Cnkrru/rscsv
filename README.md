# rscsv

A lightweight, zero-dependency CSV reading and writing library for Rust, built entirely using the standard library.

## Features

- **Zero Dependencies**: Built entirely using Rust's standard library
- **CSV Reading**: Iterator-based reader with automatic header detection
- **CSV Writing**: Generic write interface with automatic quoting and escaping
- **Flexible Configuration**: Builder pattern with flexible row lengths
- **Custom Delimiters**: Support for any delimiter (`,`, `;`, `\t`, etc.)
- **Quoting & Escaping**: Quoted fields, double-quote escaping, and escape character support
- **Comment Lines**: Skip lines starting with a comment character
- **Field Trimming**: Automatically trim leading/trailing whitespace
- **Multiple Formats**: Read/write with different quoting styles

## Quick Start

Add `rscsv` to your `Cargo.toml`:

```toml
[dependencies]
rscsv = { path = "../rscsv" }
```

### Reading CSV

```rust
use rscsv::{CsvConfig, CsvReader};
use std::io::Cursor;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data = "name,age,city\nAlice,30,NYC\nBob,25,LA\n";

    let config = CsvConfig::builder()
        .has_headers(true)
        .build();

    let mut reader = CsvReader::from_reader(Cursor::new(data))
        .with_config(config);

    let records = reader.read_all()?;

    println!("Headers: {:?}", reader.headers());
    for record in records {
        println!("{:?}", record);
    }
    Ok(())
}
```

### Writing CSV

```rust
use rscsv::{CsvConfig, CsvWriter};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut buf = Vec::new();
    let mut writer = CsvWriter::from_writer(&mut buf);

    writer.write_headers(&["name", "age"])?;
    writer.write_record(&["Alice", "30"])?;
    writer.write_record(&["Bob", "25"])?;
    writer.flush()?;

    println!("{}", String::from_utf8(buf)?);
    Ok(())
}
```

### File I/O

```rust
use rscsv::{FileReader, FileWriter};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut reader = FileReader::open("input.csv")?;
    let records = reader.read_all()?;

    let mut writer = FileWriter::create("output.csv")?;
    writer.write_record(&["a", "b", "c"])?;
    writer.flush()?;

    Ok(())
}
```

## Configuration Options

```rust
use rscsv::CsvConfig;

let config = CsvConfig::builder()
    .delimiter(b',')       // Field delimiter (default: ,)
    .quote(b'"')           // Quote character (default: ")
    .escape(b'\\')         // Escape character (optional)
    .has_headers(true)     // First row is headers
    .flexible(false)       // Allow variable row lengths
    .trim(false)           // Trim whitespace from fields
    .comment(b'#')         // Skip comment lines (optional)
    .build();
```

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `delimiter` | `u8` | `,` | Field delimiter |
| `quote` | `u8` | `"` | Quote character |
| `escape` | `Option<u8>` | `None` | Escape character |
| `has_headers` | `bool` | `false` | First row contains headers |
| `flexible` | `bool` | `false` | Allow variable row lengths |
| `trim` | `bool` | `false` | Trim whitespace from fields |
| `comment` | `Option<u8>` | `None` | Skip lines starting with this character |

## Examples

Run the basic example:

```bash
cargo run --example demo
```

Run parser tests:

```bash
cargo run --example test_parser
```

## Documentation

Generate documentation:

```bash
cargo doc --open
```

## License

MIT License

## Contributing

Contributions are welcome! Please feel free to submit issues and pull requests.

## 中文文档

中文文档请查看 [rs-docs](https://github.com/Cnkrru/rust-package/tree/main/rs-docs)