use crate::schema::*;
use diesel::prelude::*;
use serde::Serialize;

#[derive(Insertable)]
#[table_name = "todos"]
pub struct NewTodo {
    pub text: String,
}

#[derive(Debug, Queryable, Serialize)]
pub struct Todo {
    pub id: i32,
    pub text: String,
    pub done: bool,
    pub finish_timestamp: Option<chrono::DateTime<chrono::Utc>>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}
