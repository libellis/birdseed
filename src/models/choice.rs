use crate::schema::*;

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
