use diesel::pg::PgConnection;
use diesel::prelude::*;

use std::error::Error;

use rayon::prelude::*;

use indicatif::ProgressBar;

use crate::pg_pool::Pool;

use crate::models::question::{NewQuestion, Question};

/// Populates questions table with row_count random questions ensuring that each question relates to
/// an existing survey.
pub fn populate(
    pool: &Pool,
    survey_ids: &Vec<i32>,
    row_count: u32,
    bar: &ProgressBar,
) -> Result<Vec<i32>, Box<dyn Error>> {
    bar.set_message(&format!("Seeding {} questions", row_count));

    let question_ids: Vec<i32> = survey_ids
        .par_iter()
        .map(|s_id| {
            let pool = pool.clone();
            let conn = pool.get().unwrap();

            let q_title = format!("{}", fake!(Lorem.sentence(3, 7)));
            let q_type = "multiple".to_string();
            let question = create(&conn, *s_id, &q_type, &q_title);
            bar.inc(1);

            question.id
        })
        .collect();

    Ok(question_ids)
}

/// Creates a single question for the given survey id in the database
pub fn create<'a>(conn: &PgConnection, s_id: i32, q_type: &'a str, q_title: &'a str) -> Question {
    use crate::schema::questions;

    let new_question = NewQuestion {
        survey_id: s_id,
        question_type: q_type,
        title: q_title,
    };

    diesel::insert_into(questions::table)
        .values(&new_question)
        .get_result(conn)
        .expect("Error saving new question")
}
