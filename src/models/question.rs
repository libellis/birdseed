use crate::schema::*;

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
