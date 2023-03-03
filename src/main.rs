#![allow(dead_code)]

mod models;
mod schema;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use models::*;
use schema::*;
use std::env;

fn main() {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let mut connection = PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url));

    create(&mut connection);
    //finish(&mut connection);
}

fn create(connection: &mut PgConnection) {
    let new_log = NewTodo {
        text: "Create todo application".to_string(),
    };

    let inserted_row = diesel::insert_into(todos::table)
        .values(&new_log)
        .get_result::<Todo>(connection);

    println!("{:?}", inserted_row);
}

fn finish(connection: &mut PgConnection) {
    let todos = todos::dsl::todos.filter(todos::done.eq(false).and(todos::id.eq(1)));

    let updated_row = diesel::update(todos)
        .set((
            todos::done.eq(true),
            todos::finish_timestamp.eq(Some(chrono::Utc::now())),
        ))
        .get_result::<Todo>(connection);

    println!("{:?}", updated_row);
}
