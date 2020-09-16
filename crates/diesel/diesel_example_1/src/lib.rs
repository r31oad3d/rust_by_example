#[macro_use]
extern crate diesel;
extern crate dotenv;

use self::models::{NewPost, Post};
use diesel::{
    debug_query,
    pg::{Pg, PgConnection},
    prelude::*,
};
use dotenv::dotenv;
use std::env;
pub mod models;
pub mod schema;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url =
        env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_post<'a>(
    conn: &PgConnection,
    title: &'a str,
    body: &'a str,
) -> Post {
    use schema::posts;
    let new_post = NewPost { title, body };

    let insert = diesel::insert_into(posts::table).values(&new_post);
    println!("{}", debug_query::<Pg, _>(&insert));
    insert.get_result(conn).expect("Error saving new post")
}
