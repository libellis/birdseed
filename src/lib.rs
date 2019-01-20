#[macro_use]
extern crate structopt;

#[macro_use]
extern crate diesel;
extern crate dotenv;

#[macro_use]
extern crate fake;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

use std::error::Error;
use structopt::StructOpt;

pub mod models;
pub mod schema;

use self::models::{NewSurvey, NewUser, Survey, User};
use self::schema::users::dsl::*;

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
    let connection = establish_connection();
    let surveys_str = String::from("surveys");
    let users_str = String::from("users");
    match config.table.to_lowercase() {
        users_str => populate_users(&connection, config.row_count)?,
        surveys_str => populate_surveys(config.row_count)?,
        _ => panic!("That table name doesn't exist!"),
    };

    Ok(())
}

fn populate_all(conn: &PgConnection, row_count: u32) -> Result<(), Box<dyn Error>> {
    populate_users(&connection, row_count);
    populate_surveys(&connection, row_count);
}

fn populate_surveys(
    conn: &PgConnection,
    authors: Vec<String>,
    row_count: u32,
) -> Result<(), Box<dyn Error>> {
    for i in 0..row_count {
        let auth = authors[i];
        let survey_title = format!("{}", fake!(Lorem.sentence(4, 8)));
        create_survey(conn, &auth, &survey_title);
    }

    Ok(())
}

fn populate_users(conn: &PgConnection, row_count: u32) -> Result<(), Box<dyn Error>> {
    for _ in 0..row_count {
        let user = format!("{}", fake!(Internet.user_name));
        let pw = format!(
            "{}{}",
            fake!(Name.name),
            fake!(Number.between(10000, 99999))
        );
        let em = format!("{}", fake!(Internet.safe_email));
        let first = format!("{}", fake!(Name.first_name));
        let last = format!("{}", fake!(Name.last_name));

        create_user(conn, &user, &pw, &em, &first, &last);
    }

    Ok(())
}

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

pub fn create_user<'a>(
    conn: &PgConnection,
    user: &'a str,
    pw: &'a str,
    em: &'a str,
    first: &'a str,
    last: &'a str,
) -> User {
    use schema::users;

    let new_user = NewUser {
        username: user,
        password: pw,
        email: em,
        first_name: first,
        last_name: last,
        is_admin: false,
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(conn)
        .expect("Error saving new user")
}

pub fn create_survey<'a>(conn: &PgConnection, auth: &'a str, survey_title: &'a str) -> Survey {
    use schema::surveys;

    let new_survey = NewSurvey {
        author: auth,
        title: survey_title,
    };

    diesel::insert_into(surveys::table)
        .values(&new_survey)
        .get_result(conn)
        .expect("Error saving new survey")
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
