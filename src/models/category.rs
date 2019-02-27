use crate::schema::*;

#[derive(Queryable)]
pub struct Category {
    pub title: String,
}

#[derive(Insertable)]
#[table_name = "categories"]
pub struct NewCategory<'a> {
    pub title: &'a str,
}
