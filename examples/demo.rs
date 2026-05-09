use rscsv::{CsvConfig, CsvReader, CsvWriter};

fn main() {
    println!("=== rscsv CSV 库使用示例 ===\n");

    let data = vec![
        vec!["hello, world".to_string(), "say \"hi\"".to_string()],
        vec!["simple".to_string(), "data".to_string()],
    ];

    println!("1. 写入 CSV（自动处理引号转义）");
    let mut writer = CsvWriter::create("demo_output.csv").unwrap();
    writer.write_all(&data).unwrap();
    println!("   已写入 demo_output.csv\n");

    println!("2. 流式读取 CSV");
    let mut reader = CsvReader::open("demo_output.csv").unwrap();
    for result in reader.records() {
        let record = result.unwrap();
        println!("   {:?}", record);
    }
    println!();

    println!("3. 使用 Builder 配置（分号分隔）");
    let config = CsvConfig::builder().delimiter(b';').build();

    let semicolon_data = vec![
        vec!["name".to_string(), "age".to_string()],
        vec!["Alice".to_string(), "30".to_string()],
    ];

    let mut writer = CsvWriter::create("demo_semicolon.csv")
        .unwrap()
        .with_config(config);
    writer.write_all(&semicolon_data).unwrap();

    println!("4. 读取分号分隔的文件");
    let config = CsvConfig::builder().delimiter(b';').build();
    let mut reader = CsvReader::open("demo_semicolon.csv")
        .unwrap()
        .with_config(config);
    for result in reader.records() {
        let record = result.unwrap();
        println!("   {:?}", record);
    }

    println!("\n✅ 所有示例运行完成！");
}