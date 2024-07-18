use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = create::schema::storage)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct StorageRecord {
    pub storage_id: u32,
    pub storage_hash: String,
    pub storage_path: String
}