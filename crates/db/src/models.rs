use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::sources)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Source {
    pub id: i32,
    pub name: String,
    pub source: String,
    pub version: String,
}

#[derive(Queryable, Selectable)]
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


#[derive(Insertable)]
#[diesel(table_name = crate::schema::sources)]
pub struct NewSource<'a> {
    pub name: &'a str,
    pub source: &'a str,
    pub version: &'a str,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::builds)]
pub struct NewBuild<'a> {
    pub source: &'a str,
    pub version: &'a str,
    pub champion_alias: &'a str,
    pub champion_id: &'a str,
    pub content: serde_json::Value,
}
