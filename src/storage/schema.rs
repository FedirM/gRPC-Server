// @generated automatically by Diesel CLI.

diesel::table! {
    storage (storage_id) {
        storage_id -> Integer,
        storage_hash -> Varchar,
        storage_path -> Varchar,
    }
}
