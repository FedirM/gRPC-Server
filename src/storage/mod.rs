// use diesel::prelude::*;
// use diesel::mysql::MysqlConnection;
// use dotenvy::dotenv;

use chksum::sha2_256;
// use std::env;

// pub mod models;
// pub mod schema;

pub fn generate_hash(data: Vec<u8>) -> Result<String, Box<dyn std::error::Error>> {
    return match sha2_256::chksum(data) {
        Ok(value) => Ok(value.to_hex_lowercase()),
        Err(err) => Err(Box::new(err))
    }
}

// pub fn establish_connection() -> MysqlConnection {
//     dotenv.ok();

//     let db_url = env::var("DATABASE_URL").expect("DATABASE_URL didn't specified!");
//     MysqlConnection::establish(&db_url).unwrap_or_else(|| panic!("Error connecting to DB!"));
// }