use diesel::pg::PgConnection;
use diesel::prelude::*;

use std::error::Error;
use indicatif::ProgressBar;

use crate::pg_pool::Pool;

use crate::models::category::{ Category, NewCategory };

// TODO: UPDATE THIS TO POPULATE RANDOM CHOICES
pub fn populate(
    pool: &Pool,
    title: &str,
    bar: &ProgressBar,
) -> Result<String, Box<dyn Error>> {
    bar.set_message(&format!("Seeding 1 Test Category"));

    let pool = pool.clone();
    let conn = pool.get().unwrap();

    create(&conn, title);

    Ok(title.to_string())
}

pub fn create<'a>(conn: &PgConnection, title: &'a str) -> Category {
    use crate::schema::categories;

    let new_category = NewCategory { title };

    diesel::insert_into(categories::table)
        .values(&new_category)
        .get_result(conn)
        .expect("Error saving new vote")
}
