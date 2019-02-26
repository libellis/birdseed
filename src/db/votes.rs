use diesel::pg::PgConnection;
use diesel::prelude::*;

use std::error::Error;

use geojson::{ GeoJson, Geometry };
use geojson::Value::{ self, Point };

use diesel_geography::types::GeogPoint;

use rand::seq::SliceRandom;
use rand::{ thread_rng, Rng};

use rayon::prelude::*;

use indicatif::ProgressBar;

use crate::pg_pool::Pool;

use crate::models::vote::{ Vote, NewVote };

// Populates the votes table with real votes from our newly inserted random users who vote on
// choices in a semi-randomish way (not that random really)
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
pub struct GeoBox {
    pub x_range: (f64, f64),
    pub y_range: (f64, f64),
}


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

        // placeholder - randomize later
        let geo_pnt: GeogPoint = gen_rand_geo();

        let geo_val = Point(vec![geo_pnt.x, geo_pnt.y]);

        let fence_tit = get_fence_by_coords(&conn, geo_val).unwrap();

        create(&conn, c_id, author, 1, geo_pnt, &fence_tit);
        bar.inc(1);
    });

    Ok(())
}

pub fn get_fence_by_coords(conn: &PgConnection, coords: Value) -> Result<String, Box<dyn Error>> {
    use diesel::dsl::sql;
    use crate::schema::fences::dsl::*;

    let geo_json = GeoJson::from(Geometry::new(coords));

    let where_str = format!("ST_Intersects(ST_GeomFromGeoJSON('{}'), geo)", geo_json.to_string());
    
    let fence: String = fences.select(title).filter(sql(&where_str)).first(conn)?;
    
    Ok(fence)
}

/**
 * The following series of functions are very simple - each one simply creates a single
 * user/survey/question/choice/vote
 */
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
