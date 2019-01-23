use std::process;
extern crate birdseed;

use structopt::StructOpt;

fn main() {
    let config = birdseed::Birdseed::from_args();

    if let Err(e) = birdseed::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    };
}
