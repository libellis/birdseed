use super::schema::users;
use std::time::SystemTime;

#[derive(Queryable)]
pub struct User {
    pub username: String,
    pub password: String,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub photo_url: Option<String>,
    pub is_admin: bool,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub password: &'a str,
    pub email: &'a str,
    pub first_name: &'a str,
    pub last_name: &'a str,
    pub is_admin: bool,
}

#[derive(Queryable)]
pub struct Survey {
    pub id: i32,
    pub author: String,
    pub title: String,
    pub description: Option<String>,
    pub anonymous: bool,
    pub published: bool,
    pub date_posted: Option<SystemTime>,
}

#[derive(Insertable)]
#[table_name = "surveys"]
pub struct NewSurvey<'a> {
    pub author: &'a str,
    pub title: &'a str,
}
