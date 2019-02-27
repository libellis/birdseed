use diesel::pg::PgConnection;
use diesel::prelude::*;

use std::error::Error;

use rayon::prelude::*;

use indicatif::ProgressBar;

use crate::pg_pool::Pool;

use crate::models::choice::{Choice, NewChoice};

/// Populates the database with fake choices
pub fn populate(
    pool: &Pool,
    question_ids: &Vec<i32>,
    row_count: u32,
    bar: &ProgressBar,
) -> Result<Vec<i32>, Box<dyn Error>> {
    bar.set_message(&format!("Seeding {} choices", (row_count * 4)));

    let choice_ids: Vec<i32> = question_ids
        .par_iter()
        .map(|q_id| {
            // For each question, inject 4 random text choices
            (0..4)
                .into_par_iter()
                .map(|_| {
                    let pool = pool.clone();
                    let conn = pool.get().unwrap();

                    let c_title = format!("{}", fake!(Lorem.sentence(1, 4)));
                    let c_type = "text".to_string();
                    let choice = create(&conn, *q_id, &c_type, &c_title);
                    bar.inc(1);
                    choice.id
                })
                .collect()
        })
        .collect::<Vec<Vec<i32>>>()
        .concat();

    Ok(choice_ids)
}

/// Creates a single choice (for a given question) in the database.
pub fn create<'a>(conn: &PgConnection, q_id: i32, c_type: &'a str, c_title: &'a str) -> Choice {
    use crate::schema::choices;

    let new_choice = NewChoice {
        question_id: q_id,
        content_type: c_type,
        title: c_title,
    };

    diesel::insert_into(choices::table)
        .values(&new_choice)
        .get_result(conn)
        .expect("Error saving new choice")
}
