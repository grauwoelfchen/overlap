mod line;
use line::Line;

mod config;
pub use config::Config;

pub mod reader;

/// Returns overlap lines.
pub fn overlap(text: String, c: &Config) -> String {
    let mut lines: Vec<Line> = vec![];

    // counting up
    for s in text.lines() {
        let line = Line::new(s);

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
        if line.count() > 1 && !line.text().is_empty() {
            let mut s = String::new();
            s.push_str(&line.text());
            if c.with_count {
                s.push_str(&format!(" {}", line.count()));
            }
            result.push(s);
        }
    }
    result.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_overlap_returns_only_overlap_texts() {
        let c = Config::new(false);

        let text = "Hoi\nZäme!\nHoi\n".to_string();
        assert_eq!("Hoi", overlap(text, &c));

        let text = "Hoi\nZäme!\n".to_string();
        assert_eq!("", overlap(text, &c));
    }

    #[test]
    fn test_overlap_returns_with_count() {
        let text = "Hoi\nZäme!\nHoi\n".to_string();
        let c = Config::new(true);
        assert_eq!("Hoi 2", overlap(text, &c))
    }
}
