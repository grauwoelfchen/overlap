use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::thread;

/// Reads file content. This function returns only unique lines.
pub fn read_file(file: &str) -> io::Result<String> {
    let path = Path::new(file);
    if !path.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "file does not exist",
        ));
    }
    let mut f = File::open(path)?;

    let mut buffer = String::new();
    f.read_to_string(&mut buffer)?;

    // only unique lines
    let mut content: Vec<&str> = vec![];
    for buf in buffer.lines() {
        if !content.contains(&buf) {
            content.push(buf);
        }
    }

    let mut text = content.join("\n");
    text.push('\n');
    Ok(text)
}

/// Reads files using read_file in multiple threads and returns its text.
pub fn read_files(files: Vec<String>) -> String {
    let contents = Arc::new(Mutex::new("".to_string()));
    let mut handles = vec![];

    for file in files {
        let contents = Arc::clone(&contents);
        let handle = thread::spawn(move || {
            let mut data = contents.lock().unwrap();
            let mut s = data.to_string();

            s.push_str(&read_file(&file).unwrap());
            *data = s;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let result = (*contents).lock().unwrap();
    result.to_string()
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
