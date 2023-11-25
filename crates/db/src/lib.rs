pub mod models;
pub mod schema;

use std::env;

use diesel::prelude::*;
use diesel::upsert::excluded;
use diesel_async::{
    pooled_connection::{
        deadpool::{BuildError, Pool},
        AsyncDieselConnectionManager,
    },
    AsyncConnection, AsyncPgConnection, RunQueryDsl,
};
use diesel_migrations::{embed_migrations, EmbeddedMigrations};
use dotenvy::dotenv;

use models::{Build, NewSource, Source};

use crate::models::NewBuild;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub async fn establish_connection() -> Result<AsyncPgConnection, ConnectionError> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    AsyncPgConnection::establish(&database_url).await
}

pub async fn insert_source(
    conn: &mut AsyncPgConnection,
    name: String,
    source: String,
    version: String,
) -> Result<Source, diesel::result::Error> {
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
        .await
}

pub async fn insert_many_sources(
    conn: &mut AsyncPgConnection,
    list: Vec<NewSource>,
) -> Result<usize, diesel::result::Error> {
    use schema::sources::{dsl as sources_dsl, table};

    diesel::insert_into(table)
        .values(&list)
        .on_conflict(sources_dsl::source)
        .do_update()
        .set((
            sources_dsl::name.eq(excluded(sources_dsl::name)),
            sources_dsl::version.eq(excluded(sources_dsl::version)),
        ))
        .execute(conn)
        .await
}

pub async fn insert_build(
    conn: &mut AsyncPgConnection,
    source: String,
    version: String,
    champion_alias: String,
    champion_id: String,
    content: serde_json::Value,
) -> Result<Build, diesel::result::Error> {
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
        .await
}

pub async fn upsert_many_builds(
    conn: &mut AsyncPgConnection,
    list: Vec<NewBuild>,
) -> Result<usize, diesel::result::Error> {
    use schema::builds::{dsl as builds_dsl, table};

    diesel::insert_into(table)
        .values(&list)
        .on_conflict((
            builds_dsl::source,
            builds_dsl::champion_id,
            builds_dsl::champion_alias,
        ))
        .do_update()
        .set((
            builds_dsl::version.eq(excluded(builds_dsl::version)),
            builds_dsl::content.eq(excluded(builds_dsl::content)),
        ))
        .execute(conn)
        .await
}

pub fn get_db_config() -> AsyncDieselConnectionManager<diesel_async::AsyncPgConnection> {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let config = AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(db_url);
    config
}

pub type DbPool = Pool<AsyncPgConnection>;

pub fn make_db_pool() -> Result<DbPool, BuildError> {
    let config = get_db_config();
    Pool::builder(config).build()
}

pub async fn list_sources(
    pool: DbPool,
) -> anyhow::Result<Vec<Source>> {
    use schema::sources::dsl::*;

    match pool.get().await {
        Ok(mut conn) => {
            let result = sources.load::<Source>(&mut conn).await?;
            Ok(result)
        }
        Err(err) => {
            Err(anyhow::anyhow!("Error getting connection: {:?}", err))
        }
    }
}
