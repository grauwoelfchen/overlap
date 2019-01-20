use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::thread;

mod line;
use line::Line;

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
    text.push_str("\n");
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

/// Returns overlap lines.
pub fn overlap(text: String) -> String {
    let mut lines: Vec<Line> = vec![];

    // counting up
    for s in text.lines() {
        let mut line = Line::new(s);

        // this sorting is needed for searching
        lines.sort();

        match lines.binary_search_by(|a| a.cmp(&line)) {
            Ok(i) => {
                let l = &mut lines[i];
                l.up();
            },
            Err(_) => {
                lines.push(line);
            },
        };
    }

    let mut result: Vec<String> = vec![];
    for line in lines {
        if line.count() > 1 {
            result.push(line.text());
        }
    }
    result.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_file_returns_file_content() {
        let s = read_file("./Cargo.toml").unwrap();
        assert!(s.contains("overlap"));
    }

    #[test]
    fn test_overlap_returns_only_overlap_texts() {
        let text = "Hoi\nZäme!\nHoi\n".to_string();
        assert_eq!("Hoi", overlap(text));

        let text = "Hoi\nZäme!\n".to_string();
        assert_eq!("", overlap(text));
    }
}
