pub mod models;
pub mod schema;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use models::{NewSource, Source, Build};
use std::env;

use crate::models::NewBuild;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_source(conn: &mut PgConnection, name: &str, source: &str, version: &str) -> Source {
    use schema::sources;

    let new_source = NewSource {
        name,
        source,
        version,
    };
    diesel::insert_into(sources::table)
        .values(&new_source)
        .returning(Source::as_returning())
        .get_result(conn)
        .expect("Error creating new source")
}

pub fn insert_many_sources(conn: &mut PgConnection, list: Vec<NewSource>) -> usize {
    use schema::sources;

    diesel::insert_into(sources::table)
        .values(&list)
        .execute(conn)
        .expect("Error creating new source")
}

pub fn create_build(conn: &mut PgConnection, source: &str, version: &str, champion_alias: &str, champion_id: i32, content: serde_json::Value) -> Build {
    use schema::builds;

    let new_source = NewBuild {
        source,
        version,
        champion_alias,
        champion_id,
        content,
    };
    diesel::insert_into(builds::table)
        .values(&new_source)
        .returning(Build::as_returning())
        .get_result(conn)
        .expect("Error creating new build")
}

#[cfg(test)]
mod tests {
    // use super::*;
}
