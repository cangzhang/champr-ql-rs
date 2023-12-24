pub mod models;
pub mod schema;

use std::env;

use diesel::prelude::*;
use diesel::upsert::excluded;
use diesel_async::{
    pooled_connection::{
        deadpool::{BuildError, Object, Pool},
        AsyncDieselConnectionManager,
    },
    AsyncConnection, AsyncPgConnection, RunQueryDsl,
};
use dotenvy::dotenv;

use models::{Build, NewSource, Source, Log};

use crate::models::NewBuild;

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

    AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(db_url)
}

pub type DbPool = Pool<AsyncPgConnection>;

pub fn make_db_pool() -> Result<DbPool, BuildError> {
    let config = get_db_config();
    Pool::builder(config).build()
}

pub async fn get_conn(pool: DbPool) -> anyhow::Result<Object<AsyncPgConnection>> {
    match pool.get().await {
        Ok(conn) => Ok(conn),
        Err(err) => Err(anyhow::anyhow!("Error getting connection: {:?}", err)),
    }
}

pub async fn list_sources(pool: DbPool) -> anyhow::Result<Vec<Source>> {
    use schema::sources::dsl::*;

    let mut conn = get_conn(pool).await?;
    let result = sources.load::<Source>(&mut conn).await?;
    Ok(result)
}

pub async fn find_builds_by_champion_alias_and_source(
    pool: DbPool,
    champ: String,
    src: String,
) -> anyhow::Result<Build> {
    use schema::builds::dsl::*;

    let mut conn = get_conn(pool).await?;
    let result = builds
        .filter(champion_alias.eq(champ).and(source.eq(src)))
        .first::<Build>(&mut conn)
        .await?;
    Ok(result)
}

pub async fn find_builds_by_champion_id_and_source(
    pool: DbPool,
    champ_id: String,
    src: String,
) -> anyhow::Result<Build> {
    use schema::builds::dsl::*;

    let mut conn = get_conn(pool).await?;
    let result = builds
        .filter(champion_id.eq(champ_id).and(source.eq(src)))
        .first::<Build>(&mut conn)
        .await?;
    Ok(result)
}

pub async fn insert_log(conn: &mut AsyncPgConnection, action: String) -> Result<Log, diesel::result::Error> {
    use schema::logs::{dsl as logs_dsl, table};

    diesel::insert_into(table)
        .values(logs_dsl::action.eq(action))
        .returning(Log::as_returning())
        .get_result(conn)
        .await
}
