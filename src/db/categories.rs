use diesel::pg::PgConnection;
use diesel::prelude::*;

use indicatif::ProgressBar;
use std::error::Error;

use crate::pg_pool::Pool;
use rayon::prelude::*;

use crate::models::category::{Category, NewCategory};

/// Not Complete: Will soon seed random categories - for now
/// seeds a single given category.
pub fn populate(pool: &Pool, row_count: i32, bar: &ProgressBar) -> Result<Vec<String>, Box<dyn Error>> {
    bar.set_message(&format!("Seeding {} categories", row_count));


    let categories: Vec<String> = (0..row_count)
        .into_par_iter()
        .map(|_| {
            let pool = pool.clone();
            let conn = pool.get().unwrap();

            let title = format!("{}", fake!(Lorem.word));
            create(&conn, &title);
            title
        })
        .collect();


    Ok(categories)
}

/// Gets a single category from the database by the given category title(PK).
pub fn get<'a>(conn: &PgConnection, category_title: &'a str) -> Result<Category, diesel::result::Error> {
    use crate::schema::categories::dsl::*;

    categories.find(category_title).first(conn)
}

/// Gets all categories
pub fn get_all(conn: &PgConnection) -> Result<Vec<Category>, diesel::result::Error> {
    use crate::schema::categories::dsl::*;

    categories.get_results(conn)
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

// NOTE: No Update on Category for now - only one field which is PK and you can never update a PK.

/// Deletes a category based on it's title.
pub fn delete<'a>(conn: &PgConnection, category_title: &'a str) -> Result<(), Box<dyn Error>> {
    use crate::schema::categories::dsl::*;

    diesel::delete(categories.filter(title.eq(category_title))).execute(conn)?;

    Ok(())
}
