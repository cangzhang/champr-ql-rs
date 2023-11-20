// @generated automatically by Diesel CLI.

diesel::table! {
    builds (id) {
        id -> Int4,
        source -> Varchar,
        version -> Varchar,
        champion_alias -> Varchar,
        champion_id -> Varchar,
        content -> Json,
    }
}

diesel::table! {
    sources (id) {
        id -> Int4,
        name -> Varchar,
        source -> Varchar,
        version -> Varchar,
    }
}

diesel::allow_tables_to_appear_in_same_query!(builds, sources,);
