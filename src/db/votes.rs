use diesel::pg::PgConnection;
use diesel::prelude::*;

use std::error::Error;

use geojson::Value::{self, Point};
use geojson::{GeoJson, Geometry};

use diesel_geography::types::GeogPoint;

use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};

use rayon::prelude::*;

use indicatif::ProgressBar;

use crate::models::vote::{NewVote, Vote};
use crate::pg_pool::Pool;
use crate::sql_functions::*;

/// Populates the votes table with real votes from our newly inserted random users who vote on
/// choices in a semi-randomish way (not that random really)
pub fn populate(
    pool: &Pool,
    authors: &Vec<String>,
    choice_ids: &Vec<i32>,
    bar: &ProgressBar,
) -> Result<(), Box<dyn Error>> {
    bar.set_message(&format!("{} users are voting", (authors.len())));

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
    author_slice.par_iter().enumerate().for_each(|(i, rand_i)| {
        let name = &authors[*rand_i];

        if i < authors.len() - 1 {
            (1..5 as usize).into_par_iter().for_each(|j| {
                let pool = pool.clone();
                let conn = pool.get().unwrap();

                let c_id = choice_ids[choice_slice[(i + 1) * j]];

                let geo_pnt: GeogPoint = gen_rand_geo();
                let geo_val = Point(vec![geo_pnt.x, geo_pnt.y]);

                let fence_tit = get_fence_by_coords(&conn, geo_val).unwrap();

                create(&conn, c_id, name, 1, geo_pnt, &fence_tit);
                bar.inc(1);
            });
        }
    });

    Ok(())
}

#[derive(Copy, Clone, Debug)]
struct GeoBox {
    x_range: (f64, f64),
    y_range: (f64, f64),
}

/// Generates a random GeogPoint from two simple GeoBoxes bounding the city
/// of San Francisco
// Expand later to be more generic (taking in a single GeoBox and returning a random point within
// the bounds)
pub fn gen_rand_geo() -> GeogPoint {
    let mut rng = rand::thread_rng();
    let box1 = GeoBox {
        x_range: (-122.4378204345703, -122.40348815917969),
        y_range: (37.77831314799669, 37.80381638220768),
    };

    let box2 = GeoBox {
        x_range: (-122.50579833984375, -122.39250183105467),
        y_range: (37.74248523826606, 37.783740105227224),
    };

    let boxes: Vec<GeoBox> = vec![box1, box2];

    // generate random number between 0 and 1 and round to figure out which index to pick
    // then generate random ranges between the bounds for that box and return a new GeogPoint
    let index_choice = rng.gen::<f64>().round() as usize;
    let choosen_box = boxes[index_choice].clone();

    let mut rng1 = rand::thread_rng();
    let mut rng2 = rand::thread_rng();
    GeogPoint {
        x: rng1.gen_range(choosen_box.x_range.0, choosen_box.x_range.1),
        y: rng2.gen_range(choosen_box.y_range.0, choosen_box.y_range.1),
        srid: Some(4326),
    }
}

/// Populates randomized icecream votes with a list of users, and choice ids for the icecream
/// choices for them to randomly pick between when "voting"
pub fn populate_icecream(
    pool: &Pool,
    authors: &Vec<String>,
    choice_ids: &Vec<i32>,
    bar: &ProgressBar,
) -> Result<(), Box<dyn Error>> {
    bar.set_message(&format!("{} users are voting", (authors.len())));

    authors.par_iter().for_each(|author| {
        let mut rng = thread_rng();
        let pool = pool.clone();
        let conn = pool.get().unwrap();

        let c_id = choice_ids[rng.gen_range(0, 3) as usize];

        let geo_pnt: GeogPoint = gen_rand_geo();

        let geo_val = Point(vec![geo_pnt.x, geo_pnt.y]);

        let fence_tit = get_fence_by_coords(&conn, geo_val).unwrap();

        create(&conn, c_id, author, 1, geo_pnt, &fence_tit);
        bar.inc(1);
    });

    Ok(())
}

/// Returns the title of a fence (usually a neighboord) that the given coordinates falls into.
pub fn get_fence_by_coords(conn: &PgConnection, coords: Value) -> Result<String, Box<dyn Error>> {
    use crate::schema::fences::dsl::*;

    let geo_json = GeoJson::from(Geometry::new(coords)).to_string();

    let fence: String = fences
        .select(title)
        .filter(ST_Intersects(ST_GeomFromGeoJSON(geo_json), geo))
        .first(conn)?;

    Ok(fence)
}

/// Returns a vote by username and choice id
pub fn get<'a>(conn: &PgConnection, user: &'a str, c_id: i32) -> Result<Vote, diesel::result::Error> {
    use crate::schema::votes::dsl::*;

    votes.filter(username.eq(user)).filter(choice_id.eq(c_id)).first(conn)
}

// NOTE: Fundamental problem with get_all is that we are renaming so many fields and creating so
// many custom titles that it won't fit within the constraints of our Vote model - and by that
// logic it's not actaully returning votes anyways.  Should this be a view?

// /// Returns all votes by a question id
// pub fn get_all(
//     conn: &PgConnection,
//     ques_id: i32,
// ) -> Result<Vec<Vote>, diesel::result::Error> {
//     use crate::schema::choices::dsl::*;
//     use crate::schema::votes::dsl::*;
//     use crate::schema::questions::dsl::*;
//     use crate::schema::questions;
//     use crate::schema::choices;
//     use crate::schema::choices::dsl::id as q_id;
//     use diesel::dsl::sum;

//     questions.inner_join(choices).select((sum(score), questions::dsl::title, choices::dsl::title)).filter(q_id.eq(ques_id)).get_results(conn)
// }

/// Casts a single vote in the database for the user (name) supplied
pub fn create<'a>(
    conn: &PgConnection,
    c_id: i32,
    name: &'a str,
    points: i32,
    geo_pnt: GeogPoint,
    fence_tit: &'a str,
) -> Vote {
    use crate::schema::votes;

    let new_vote = NewVote {
        choice_id: c_id,
        username: name,
        score: points,
        geo: geo_pnt,
        fence_title: fence_tit,
    };

    diesel::insert_into(votes::table)
        .values(&new_vote)
        .get_result(conn)
        .expect("Error saving new vote")
}

// No update because we decided that you should not be able to change a vote after it's been cast

/// Deletes a vote based on a choice id and username.
pub fn delete<'a>(conn: &PgConnection, c_id: i32, user: &'a str) -> Result<(), Box<dyn Error>> {
    use crate::schema::votes::dsl::*;

    diesel::delete(votes.filter(choice_id.eq(c_id)).filter(username.eq(user))).execute(conn)?;

    Ok(())
}
