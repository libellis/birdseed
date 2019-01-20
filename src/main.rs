use birdseed;
use structopt::StructOpt;

fn main() {
    let config = birdseed::Birdseed::from_args();
    birdseed::run(config).unwrap();
}
