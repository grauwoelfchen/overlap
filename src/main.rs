//! awk '{d[$1]++} END {for (n in d) {print n,d[n]}}' < FILE
//!
//! # Examples
//!
//! ```zsh
//! % overlap --file FILE --file FILE
//! ```
use std::path::Path;

#[macro_use]
extern crate clap;
use clap::{Arg, App};

mod config;
use config::Config;

fn main() {
    let matches = App::new("Overlap")
        .version(crate_version!())
        .about("A tool shows overlap text in files")
        .arg(
            Arg::with_name("file")
                .long("file")
                .short("f")
                .value_name("FILE")
                .help("Sets a file path")
                .multiple(true)
                .required(true)
                .takes_value(true),
        )
        .get_matches();

    let mut files: Vec<String> = vec![];
    if let Some(v) = matches.values_of("file") {
        for file in v {
            if !Path::new(file).exists() {
                eprintln!("No such file: {}", file);
                std::process::exit(1);
            } else {
                files.push(file.to_string());
            }
        }
    }

    let c = Config::new();
    if !c.is_valid() {
        eprintln!("Usage: overlap [<OPTION>] <FILE> <FILE> ...");
        std::process::exit(1);
    }

    // TODO: use config

    let text = overlap::read_files(files);
    if !text.is_empty() {
        let result = overlap::overlap(text);
        if !result.is_empty() {
            println!("{}", result);
        }
    }
}
