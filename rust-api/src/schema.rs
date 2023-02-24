// @generated automatically by Diesel CLI.

diesel::table! {
    users {
        id -> Integer,
        name -> Varchar,
        email -> Varchar,
        password -> Varchar,
    }
}
