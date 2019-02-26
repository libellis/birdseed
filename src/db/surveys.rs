use diesel::pg::PgConnection;
use diesel::prelude::*;

use std::error::Error;

use rayon::prelude::*;

use indicatif::ProgressBar;

use crate::pg_pool::Pool;

use crate::models::survey::{ Survey, NewSurvey };


// Populates surveys table with row_count random surveys making sure each survey is being created
// by an existing user
pub fn populate(
    pool: &Pool,
    authors: &Vec<String>,
    row_count: u32,
    bar: &ProgressBar,
) -> Result<Vec<i32>, Box<dyn Error>> {
    bar.set_message(&format!("Seeding {} surveys", row_count));

    let survey_ids: Vec<i32> = authors
        .par_iter()
        .map(|auth| {
            let pool = pool.clone();
            let conn = pool.get().unwrap();

            let survey_title = format!("{}", fake!(Lorem.sentence(4, 8)));

            // TODO: Change this later to not be a static category field
            let cat = "TestCategory";

            let survey = create(&conn, &auth, &survey_title, cat);
            bar.inc(1);

            survey.id
        })
        .collect();

    Ok(survey_ids)
}

pub fn create<'a>(
    conn: &PgConnection,
    auth: &'a str,
    survey_title: &'a str,
    cat: &'a str,
) -> Survey {
    use crate::schema::surveys;

    let new_survey = NewSurvey {
        author: auth,
        title: survey_title,
        published: true,
        category: cat,
    };

    diesel::insert_into(surveys::table)
        .values(&new_survey)
        .get_result(conn)
        .expect("Error saving new survey")
}
