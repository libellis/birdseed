#[macro_use]
extern crate structopt;

use structopt::StructOpt;

use std::error::Error;

#[derive(StructOpt, Debug)]
#[structopt(name = "birdseed", about = "the libellis database seeder")]
/// You can use birdseed to seed a libellis table with junk data!
pub struct Config {
    /// Table name to inject data into
    #[structopt(long = "table", short = "t")]
    table: String,

    /// How many rows to inject
    #[structopt(long = "rowcount", short = "r")]
    row_count: u32,
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let surveys = String::from("surveys");
    match config.table.to_lowercase() {
        surveys => populate_surveys(config.row_count)?,
        _ => panic!("That table name doesn't exist!"),
    };

    Ok(())
}

fn populate_surveys(row_count: u32) -> Result<(), Box<dyn Error>> {
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
