use crate::schema::*;
use diesel_geography::types::GeogPoint;
use std::time::SystemTime;

#[derive(Queryable)]
pub struct Vote {
    pub choice_id: i32,
    pub username: String,
    pub score: i32,
    pub geo: GeogPoint,
    pub fence_title: String,
    pub date_voted: SystemTime,
}

#[derive(Insertable)]
#[table_name = "votes"]
pub struct NewVote<'a> {
    pub choice_id: i32,
    pub username: &'a str,
    pub score: i32,
    pub geo: GeogPoint,
    pub fence_title: &'a str,
}
