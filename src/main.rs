use std::env;
use std::fmt;

#[allow(dead_code)]
#[derive(Debug)]
struct Sizes {
    bytes: String,
    kilobytes: String,
    megabytes: String,
    gigabytes: String,
}

impl Sizes {
    fn new(bytes: u64) -> Self {
        Sizes {
            bytes: FileSize::Bytes(bytes).to_string(),
            kilobytes: FileSize::Kilobytes(bytes as f64 / 1_000.0).to_string(),
            megabytes: FileSize::Megabytes(bytes as f64 / 1_000_000.0).to_string(),
            gigabytes: FileSize::Gigabytes(bytes as f64 / 1_000_000_000.0).to_string()
        }
    }
}

#[derive(Debug)]
enum FileSize {
    Bytes(u64),
    Kilobytes(f64),
    Megabytes(f64),
    Gigabytes(f64),
}

impl fmt::Display for FileSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FileSize::Bytes(b) => write!(f, "{:.2} bytes", b),
            FileSize::Kilobytes(kb) => write!(f, "{:.2} kilobytes", kb),
            FileSize::Megabytes(mb) => write!(f, "{:.2} megabytes", mb),
            FileSize::Gigabytes(gb) => write!(f, "{:.2} gigabytes", gb),
        }
    }
}

fn format_size(size: u64, filetype: &str) -> String {
    let bytes = match filetype {
        "byte" => size,
        "kb" => (size as f64 * 1_000.0) as u64,
        "mb" => (size as f64 * 1_000_000.0) as u64,
        _ => (size as f64 * 1_000_000_000.0) as u64,
    };
    let result = Sizes::new(bytes);
    format!("{:?}", result)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];
    let converted_input: Vec<&str> = input.split(" ").collect();
    let size = match converted_input[0].parse() {
        Ok(val) => val,
        Err(_) => {
            eprintln!("Invalid size value");
            return;
        }
    };
    let filetype = match converted_input[1].to_lowercase().as_str() {
        "b" | "byte" => "byte",
        "kb" => "kb",
        "mb" => "mb",
        "gb" => "gb",
        _ => {
            eprintln!("Invalid unit. Use 'b', 'kb', 'mb', or 'gb'.");
            return;
        }
    };
    let result = format_size(size,filetype);
    println!("{}",result);
}