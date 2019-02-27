use crate::schema::*;
use diesel_geography::types::GeogPoint;

#[derive(Queryable)]
pub struct Fence {
    pub title: String,
    pub geo_level: i32,
    pub geo: GeogPoint,
}

#[derive(Insertable)]
#[table_name = "fences"]
pub struct NewFence<'a> {
    pub title: &'a str,
    pub geo_level: i32,
    pub geo: GeogPoint,
}
