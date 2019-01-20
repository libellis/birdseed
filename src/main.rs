use birdseed;
use structopt::StructOpt;

fn main() {
    let config = birdseed::Config::from_args();
    birdseed::run(config).unwrap();
}
