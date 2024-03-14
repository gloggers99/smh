// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Text,
        password -> Text,
        admin -> Bool,
        created -> Timestamp,
        logged -> Timestamp,
    }
}
