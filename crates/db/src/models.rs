use serde_derive::Deserialize;
use serde_derive::Serialize;
use diesel::prelude::*;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[derive(Queryable, Selectable, Identifiable)]
#[diesel(table_name = crate::schema::sources)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Source {
    pub id: i32,
    pub name: String,
    pub source: String,
    pub version: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[derive(Queryable, Selectable, Identifiable)]
#[diesel(table_name = crate::schema::builds)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Build {
    pub id: i32,
    pub source: String,
    pub version: String,
    pub champion_alias: String,
    pub champion_id: String,
    pub content: serde_json::Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[derive(Insertable)]
#[diesel(table_name = crate::schema::sources)]
pub struct NewSource {
    pub name: String,
    pub source: String,
    pub version: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[derive(Insertable)]
#[diesel(table_name = crate::schema::builds)]
pub struct NewBuild {
    pub source: String,
    pub version: String,
    pub champion_alias: String,
    pub champion_id: String,
    pub content: serde_json::Value,
}
