//! awk '{d[$1]++} END {for (n in d) {print n,d[n]}}' < FILE
//!
//! # Examples
//!
//! ```zsh
//! % overlap --help
//! ```
extern crate structopt;

extern crate overlap;

use structopt::StructOpt;

mod config;
use config::Config;

/// command options.
#[derive(StructOpt)]
#[structopt(
    name = "overlap",
    about = "\nawk '{d[$1]++} END {for (n in d) {print n,d[n]}}' < FILE"
)]
pub struct Opts {
    /// Filename
    #[structopt(short = "f", long = "file")]
    file: String,
}

fn main() {
    let opts = Opts::from_args();

    let c = Config::new(opts);
    if !c.is_valid() {
        eprintln!("Usage: overlap [<OPTION>] <FILE> <FILE> ...");
        std::process::exit(1);
    }

    let s = overlap::read_file(&c.file);
    match s {
        Ok(content) => {
            // TODO
            overlap::print(&content)
        }
        Err(e) => panic!(e),
    };
}
