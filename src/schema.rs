// @generated automatically by Diesel CLI.

diesel::table! {
    exchanges (id) {
        id -> Nullable<Integer>,
        name -> Text,
        url -> Text,
        created_at -> Timestamp,
    }
}
