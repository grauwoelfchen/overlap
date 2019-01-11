use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;
use std::process;

/// Read file content.
pub fn read_file(file: &str) -> io::Result<String> {
    let path = Path::new(file);
    if !path.exists() {
        eprintln!("file does not exist");
        process::exit(1);
    }
    let mut f = File::open(path)?;

    let mut buffer = String::new();
    f.read_to_string(&mut buffer)?;

    Ok(buffer)
}

/// Print file content.
pub fn print(content: &str) {
    println!("{}", content)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_file_returns_file_content() {
        let s = read_file("./Cargo.toml").unwrap();
        assert!(s.contains("overlap"));
    }
}
