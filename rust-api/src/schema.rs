// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Integer,
        name -> Varchar,
        email -> Varchar,
        password -> Varchar,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}
