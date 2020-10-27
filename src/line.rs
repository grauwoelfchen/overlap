use std::cmp::Ordering;

#[derive(Eq)]
pub struct Line {
    count: i64,
    text: String,
}

impl Default for Line {
    fn default() -> Line {
        Line {
            count: 1,
            text: "".to_string(),
        }
    }
}
impl PartialOrd for Line {
    fn partial_cmp(&self, other: &Line) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Line {
    fn cmp(&self, other: &Line) -> Ordering {
        // does not care count value
        self.text.cmp(&other.text)
    }
}

impl PartialEq for Line {
    fn eq(&self, other: &Line) -> bool {
        self.text == other.text
    }
}

impl Line {
    pub fn new(text: &str) -> Self {
        Line {
            text: text.to_string(),
            ..Default::default()
        }
    }

    pub fn count(&self) -> i64 {
        self.count
    }

    pub fn text(&self) -> String {
        self.text.to_owned()
    }

    pub fn up(&mut self) {
        self.count += 1;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_line_defaults() {
        let line = Line {
            ..Default::default()
        };
        assert_eq!(1, line.count());
        assert_eq!("".to_string(), line.text());
    }

    #[test]
    fn test_line_eq() {
        let line_a = Line::new("foo");
        let line_b = Line {
            count: 9,
            text: "foo".to_string(),
        };
        assert!(line_a == line_b);

        let line_c = Line {
            count: 9,
            text: "bar".to_string(),
        };
        assert!(line_a != line_c);
        assert!(line_b != line_c);
    }

    #[test]
    fn test_new() {
        let line = Line::new("hello");
        assert_eq!("hello".to_string(), line.text());
    }

    #[test]
    fn test_count() {
        let line = Line {
            count: 9,
            text: "text".to_string(),
        };
        assert_eq!(9, line.count());
    }

    #[test]
    fn test_text() {
        let line = Line {
            count: 9,
            text: "text".to_string(),
        };
        assert_eq!("text".to_string(), line.text());
    }

    #[test]
    fn test_up() {
        let mut line = Line::new("count");
        assert_eq!(1, line.count());
        line.up();
        assert_eq!(2, line.count());
        line.up();
        assert_eq!(3, line.count());
    }
}
