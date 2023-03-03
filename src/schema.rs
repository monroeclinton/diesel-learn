// @generated automatically by Diesel CLI.

diesel::table! {
    todos (id) {
        id -> Int4,
        text -> Text,
        done -> Bool,
        finish_timestamp -> Nullable<Timestamptz>,
        timestamp -> Timestamptz,
    }
}
