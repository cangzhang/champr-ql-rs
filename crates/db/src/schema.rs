// @generated automatically by Diesel CLI.

diesel::table! {
    builds (id) {
        id -> Int4,
        source -> Varchar,
        version -> Varchar,
        champion_alias -> Varchar,
        champion_id -> Int4,
        content -> Json,
    }
}
