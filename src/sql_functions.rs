use diesel::sql_types::{Bool, Text};
use diesel_geography::sql_types::Geography;

sql_function!(fn ST_GeomFromGeoJSON(geojson: Text) -> Geography);
sql_function!(fn ST_AsGeoJSON(geom: Geography) -> Text);
sql_function!(fn ST_Intersects(geom1: Geography, geom2: Geography) -> Bool);
