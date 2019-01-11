use Opts;

pub struct Config {
    pub file: String
}

impl Config {
    pub fn new(opts: Opts) -> Self {
        Config {
            file: opts.file
        }
    }

    pub fn is_valid(&self) -> bool {
        // TODO
        true
    }
}
