// #![allow(proc_macro_derive_resolution_fallback)]

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

#[derive(StructOpt, Debug)]
#[structopt(name = "birdseed", about = "the libellis database seeder")]
/// You can use birdseed to seed a libellis table with junk data!
pub enum Birdseed {
    #[structopt(name = "feed")]
    /// Injects random data into all tables
    Feed {
        /// How many rows to inject
        #[structopt(short = "r", long = "rows", default_value = "1000")]
        row_count: u32,
    },

    #[structopt(name = "clear")]
    /// Clears all tables in libellis database
    Clear,
}

pub fn run(config: Birdseed) -> Result<(), Box<dyn Error>> {
    let connection = establish_connection();

    match config {
        Birdseed::Feed { row_count } => populate_all(&connection, row_count),
        Birdseed::Clear => clear_all(&connection),
    }
}

fn populate_all(conn: &PgConnection, row_count: u32) -> Result<(), Box<dyn Error>> {
    let usernames = populate_users(&conn, row_count).unwrap();
    populate_surveys(&conn, &usernames, row_count)?;
    Ok(())
}

fn clear_all(conn: &PgConnection) -> Result<(), Box<dyn Error>> {
    use schema::*;

    diesel::delete(users::table).execute(conn);
    diesel::delete(surveys::table).execute(conn);

    Ok(())
}

fn populate_surveys(
    conn: &PgConnection,
    authors: &Vec<String>,
    row_count: u32,
) -> Result<(), Box<dyn Error>> {
    for i in 0..row_count as usize {
        let auth = &authors[i];
        let survey_title = format!("{}", fake!(Lorem.sentence(4, 8)));
        create_survey(conn, auth, &survey_title);
    }

    Ok(())
}

fn populate_users(conn: &PgConnection, row_count: u32) -> Result<Vec<String>, Box<dyn Error>> {
    let mut usernames = Vec::new();
    for _ in 0..row_count {
        let user = format!(
            "{}{}",
            fake!(Internet.user_name),
            fake!(Number.between(90, 9999))
        );
        let pw = format!(
            "{}{}",
            fake!(Name.name),
            fake!(Number.between(10000, 99999))
        );
        let em = format!("{}@gmail.com", user);
        let first = format!("{}", fake!(Name.first_name));
        let last = format!("{}", fake!(Name.last_name));

        create_user(conn, &user, &pw, &em, &first, &last);
        usernames.push(user);
    }

    Ok(usernames)
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
