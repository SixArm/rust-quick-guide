use diesel::prelude::*;
use crate::schema::posts;

#[derive(Debug, Queryable, Identifiable, AsChangeset)]
pub struct Post {
    pub id: i32,
    pub title: String,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = posts)]
pub struct NewPost<'a> {
    pub id: i32,
    pub title: &'a str,
}
