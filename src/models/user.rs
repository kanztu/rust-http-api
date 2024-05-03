use diesel::prelude::*;
use serde::Serialize;

diesel::table! {
    users (id) {
        id -> Uuid,
        username -> Text,
    }
}

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = users)]
pub struct User {
    pub id: String,
    pub username: String,
}
