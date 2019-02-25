use super::schema::*;
use diesel_geography::types::GeogPoint;
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
    pub date_posted: SystemTime,
    pub category: String,
}

#[derive(Queryable)]
pub struct Category {
    pub title: String,
}

#[derive(Insertable)]
#[table_name = "surveys"]
pub struct NewSurvey<'a> {
    pub author: &'a str,
    pub title: &'a str,
    pub published: bool,
    pub category: &'a str,
}

#[derive(Queryable)]
pub struct Question {
    pub id: i32,
    pub survey_id: i32,
    pub question_type: String,
    pub title: String,
}

#[derive(Insertable)]
#[table_name = "questions"]
pub struct NewQuestion<'a> {
    pub survey_id: i32,
    pub question_type: &'a str,
    pub title: &'a str,
}

#[derive(Queryable)]
pub struct Choice {
    pub id: i32,
    pub question_id: i32,
    pub content: Option<String>,
    pub content_type: String,
    pub title: String,
}

#[derive(Insertable)]
#[table_name = "choices"]
pub struct NewChoice<'a> {
    pub question_id: i32,
    pub content_type: &'a str,
    pub title: &'a str,
}

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

#[derive(Insertable)]
#[table_name = "categories"]
pub struct NewCategory<'a> {
    pub title: &'a str,
}
