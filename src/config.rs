pub struct Config {
    pub with_count: bool,
}

impl Config {
    pub fn new() -> Self {
        Config { with_count: false }
    }

    pub fn is_valid(&self) -> bool {
        // TODO
        true
    }
}
