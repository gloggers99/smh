// @generated automatically by Diesel CLI.

diesel::table! {
    posts (id) {
        id -> Int4,
        created -> Timestamp,
        title -> Text,
        author -> Text,
        description -> Text,
        content -> Text,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        email -> Text,
        username -> Text,
        password -> Text,
        admin -> Bool,
        created -> Timestamp,
        logged -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    posts,
    users,
);
