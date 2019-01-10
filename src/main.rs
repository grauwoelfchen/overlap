//! awk '{d[$1]++} END {for (n in d) {print n,d[n]}}' < FILE
//!
//! # Examples
//!
//! ```zsh
//! % overlap --help
//! ```
extern crate structopt;

use structopt::StructOpt;

mod config;
use config::Config;

/// command options.
#[derive(Debug, StructOpt)]
#[structopt(
    name = "overlap",
    about = "\nawk '{d[$1]++} END {for (n in d) {print n,d[n]}}' < FILE"
)]
pub struct Opts {}

fn main() {
    let opts = Opts::from_args();
    let c = Config::new(opts);
    if !c.is_valid() {
        eprintln!("Usage: overlap [<OPTION>] <FILE> <FILE> ...");
        std::process::exit(1);
    }

    println!("TODO");
}
