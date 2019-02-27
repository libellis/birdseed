use diesel::pg::PgConnection;
use diesel::prelude::*;

use std::error::Error;

use geojson::GeoJson;

use crate::pg_pool::Pool;

/// takes a geoJSON string and loads it into the database in a structured way - in the process
/// converting from geoJSON to geom psql type
pub fn load_geo_data(pool: &Pool, geojson_str: &String) -> Result<(), Box<dyn Error>> {
    match geojson_str.parse::<GeoJson>().unwrap() {
        GeoJson::FeatureCollection(feat_col) => {
            (for feature in feat_col.features {
                let pool = pool.clone();
                let conn = pool.get().unwrap();
                let property = feature.properties.unwrap();
                let title = property.get("nhood").unwrap().to_string();

                // for some reason serde Value types wrap their strings in double quotes - this
                // removes the quotes - look into if there's a more natural way to handle this
                let trimmed_title = title.split('"').collect::<Vec<&str>>()[1];
                let geo_level = 1;
                let geojson = feature.geometry.unwrap();

                create(&conn, trimmed_title, geo_level, GeoJson::from(geojson))?;
            })
        }
        _ => (),
    }

    Ok(())
}

/// Creates a single fence in the database, and in the process calls the ST_GeomFromGeoJSON postgis
/// function to turn the supplied geoJSON into a geom type for data storage.
// doesn't return a fence yet due to complication with transforming geo back to json
// we don't need it yet at this stage at least
pub fn create<'a>(
    conn: &PgConnection,
    tle: &'a str,
    geo_lvl: i32,
    geo_json: GeoJson,
) -> Result<(), Box<dyn Error>> {
    use crate::schema::fences::dsl::*;
    use diesel::dsl::{insert_into, sql};

    let geo_func_call = format!("ST_GeomFromGeoJSON('{}')", geo_json.to_string());

    insert_into(fences)
        .values((
            title.eq(tle),
            geo_level.eq(geo_lvl),
            geo.eq(sql(&geo_func_call)),
        ))
        .execute(conn)?;

    Ok(())
}
