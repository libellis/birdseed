#![allow(proc_macro_derive_resolution_fallback)]

#[macro_use]
extern crate structopt;

#[macro_use]
extern crate diesel;
extern crate dotenv;

#[macro_use]
extern crate fake;

extern crate rand;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

use std::error::Error;
use structopt::StructOpt;

use rand::seq::SliceRandom;
use rand::thread_rng;

pub mod models;
pub mod schema;

use self::models::{
    Choice, NewChoice, NewQuestion, NewSurvey, NewUser, NewVote, Question, Survey, User, Vote,
};

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
    let survey_ids = populate_surveys(&conn, &usernames, row_count)?;
    let question_ids = populate_questions(&conn, &survey_ids, row_count)?;
    let choice_ids = populate_choices(&conn, &question_ids, row_count)?;
    populate_votes(&conn, &usernames, &choice_ids)?;
    Ok(())
}

fn clear_all(conn: &PgConnection) -> Result<(), Box<dyn Error>> {
    use schema::*;

    diesel::delete(questions::table).execute(conn)?;
    diesel::delete(surveys::table).execute(conn)?;
    diesel::delete(users::table).execute(conn)?;

    Ok(())
}

fn populate_surveys(
    conn: &PgConnection,
    authors: &Vec<String>,
    row_count: u32,
) -> Result<Vec<i32>, Box<dyn Error>> {
    let mut survey_ids = Vec::new();
    for i in 0..row_count as usize {
        let auth = &authors[i];
        let survey_title = format!("{}", fake!(Lorem.sentence(4, 8)));
        let survey = create_survey(conn, auth, &survey_title);
        survey_ids.push(survey.id);
    }

    Ok(survey_ids)
}

fn populate_questions(
    conn: &PgConnection,
    survey_ids: &Vec<i32>,
    row_count: u32,
) -> Result<Vec<i32>, Box<dyn Error>> {
    let mut question_ids = Vec::new();
    for i in 0..row_count as usize {
        let s_id = survey_ids[i];
        let q_title = format!("{}", fake!(Lorem.sentence(3, 7)));
        let q_type = "multiple".to_string();
        let question = create_question(conn, s_id, &q_type, &q_title);
        question_ids.push(question.id);
    }

    Ok(question_ids)
}

fn populate_choices(
    conn: &PgConnection,
    question_ids: &Vec<i32>,
    row_count: u32,
) -> Result<Vec<i32>, Box<dyn Error>> {
    let mut choice_ids = Vec::new();
    for i in 0..row_count as usize {
        let q_id = question_ids[i];
        // For each question, inject 4 random text choices
        for _ in 0..4 {
            let c_title = format!("{}", fake!(Lorem.sentence(1, 4)));
            let c_type = "text".to_string();
            let choice = create_choice(conn, q_id, &c_type, &c_title);
            choice_ids.push(choice.id);
        }
    }

    Ok(choice_ids)
}

fn populate_votes(
    conn: &PgConnection,
    authors: &Vec<String>,
    choice_ids: &Vec<i32>,
) -> Result<(), Box<dyn Error>> {
    // Create vectors of idx and shuffle them
    let mut rng = thread_rng();
    let mut choice_idxs: Vec<usize> = (0..choice_ids.len()).collect();
    let choice_slice: &mut [usize] = &mut choice_idxs;
    let mut author_idxs: Vec<usize> = (0..authors.len()).collect();
    let author_slice: &mut [usize] = &mut author_idxs;
    choice_slice.shuffle(&mut rng);
    author_slice.shuffle(&mut rng);

    // For each round up randomize the choice and the author voting
    // on the choice
    for i in 0..authors.len() - 1 {
        let name = &authors[author_slice[i]];
        for j in 1..=4 {
            let c_id = choice_ids[choice_slice[(i + 1) * j]];
            create_vote(conn, c_id, name, 1);
        }
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

pub fn create_question<'a>(
    conn: &PgConnection,
    s_id: i32,
    q_type: &'a str,
    q_title: &'a str,
) -> Question {
    use schema::questions;

    let new_question = NewQuestion {
        survey_id: s_id,
        type_: q_type,
        title: q_title,
    };

    diesel::insert_into(questions::table)
        .values(&new_question)
        .get_result(conn)
        .expect("Error saving new question")
}

pub fn create_choice<'a>(
    conn: &PgConnection,
    q_id: i32,
    c_type: &'a str,
    c_title: &'a str,
) -> Choice {
    use schema::choices;

    let new_choice = NewChoice {
        question_id: q_id,
        type_: c_type,
        title: c_title,
    };

    diesel::insert_into(choices::table)
        .values(&new_choice)
        .get_result(conn)
        .expect("Error saving new choice")
}

pub fn create_vote<'a>(conn: &PgConnection, c_id: i32, name: &'a str, points: i32) -> Vote {
    use schema::votes;

    let new_vote = NewVote {
        choice_id: c_id,
        username: name,
        score: points,
    };

    diesel::insert_into(votes::table)
        .values(&new_vote)
        .get_result(conn)
        .expect("Error saving new vote")
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
