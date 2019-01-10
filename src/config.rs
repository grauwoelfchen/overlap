use Opts;

pub struct Config {
    pub opts: Opts,
}

impl Config {
    pub fn new(opts: Opts) -> Self {
        Config { opts }
    }

    pub fn is_valid(&self) -> bool {
        // TODO
        true
    }
}
