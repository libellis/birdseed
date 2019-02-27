use crate::schema::*;
use std::time::SystemTime;

#[derive(Serialize, Deserialize, Queryable)]
pub struct Survey {
    pub id: i32,
    pub author: String,
    pub title: String,
    pub description: Option<String>,
    pub anonymous: bool,
    pub published: bool,
    pub date_posted: SystemTime,
    pub category: String,
}

#[derive(Insertable)]
#[table_name = "surveys"]
pub struct NewSurvey<'a> {
    pub author: &'a str,
    pub title: &'a str,
    pub published: bool,
    pub category: &'a str,
}
