use diesel::pg::PgConnection;
use diesel::prelude::*;

use indicatif::ProgressBar;
use std::error::Error;

use crate::pg_pool::Pool;

use crate::models::category::{Category, NewCategory};

/// Not Complete: Will soon seed random categories - for now
/// seeds a single given category.
// TODO: UPDATE THIS TO POPULATE RANDOM CHOICES
pub fn populate(pool: &Pool, title: &str, bar: &ProgressBar) -> Result<String, Box<dyn Error>> {
    bar.set_message(&format!("Seeding 1 Test Category"));

    let pool = pool.clone();
    let conn = pool.get().unwrap();

    create(&conn, title);

    Ok(title.to_string())
}

/// Creates a category in the database by the given title.
pub fn create<'a>(conn: &PgConnection, title: &'a str) -> Category {
    use crate::schema::categories;

    let new_category = NewCategory { title };

    diesel::insert_into(categories::table)
        .values(&new_category)
        .get_result(conn)
        .expect("Error saving new vote")
}
