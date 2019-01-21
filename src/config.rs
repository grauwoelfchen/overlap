pub struct Config {
    pub with_count: bool,
}

impl Config {
    pub fn new(with_count: bool) -> Self {
        Config { with_count }
    }

    pub fn is_valid(&self) -> bool {
        // TODO
        true
    }
}
