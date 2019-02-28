use diesel::pg::PgConnection;
use diesel::prelude::*;

use std::error::Error;

use rayon::prelude::*;

use indicatif::ProgressBar;

use crate::models::user::{NewUser, User};
use crate::Pool;

/// Populates users table with row_count random users.
pub fn populate(
    pool: &Pool,
    row_count: u32,
    bar: &ProgressBar,
) -> Result<Vec<String>, Box<dyn Error>> {
    bar.set_message(&format!("Seeding {} users", row_count));

    let usernames: Vec<String> = (0..row_count)
        .into_par_iter()
        .map(|_| {
            let pool = pool.clone();

            let conn = pool.get().unwrap();

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

            create(&conn, &user, &pw, &em, &first, &last);
            bar.inc(1);

            user
        })
        .collect();

    Ok(usernames)
}

/// Creates a single user in the database WITHOUT hashing the password (for quickly seeding junk
/// users).
pub fn create<'a>(
    conn: &PgConnection,
    user: &'a str,
    pw: &'a str,
    em: &'a str,
    first: &'a str,
    last: &'a str,
) -> User {
    use crate::schema::users;

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