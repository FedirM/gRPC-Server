
// use std::io::Read;

// use models::StorageRecord;
// use proto::{
//     storage_server::{Storage, StorageServer},
//     GetRecordRequest, GetRecordResponse
// };

// use tonic::{transport::Server, Request, Response, Status};

// mod proto {
//     tonic::include_proto!("storage");
// }

// #[derive(Debug, Default)]
// struct StorageService {}

// #[tonic::async_trait]
// impl Storage for StorageService {
//     async fn get_record(
//         &self,
//         request: Request<GetRecordRequest>,
//     ) -> Result<Response<GetRecordResponse>, Status> {
//         println!("Get new request: {:#?}", request);

//         let req = request.into_inner();
//         println!("Inner object: {:#?}", req);

//         let reply = GetRecordResponse {
//             hash: "kek".to_string(),
//             file: b"lol".to_vec()
//         };

//         Ok(Response::new(reply))
//     }
// }


mod storage;
use storage::generate_hash;


use std::{fs::File, io::Read};

fn main() {


    let mut file = File::open("foo.txt").unwrap();
    let mut buf = Vec::with_capacity(100);
    match file.read_to_end(&mut buf) {
        Ok(n) => {
            println!("Successfully read {} bytes", n);
            println!("Hash: {}", generate_hash(buf).unwrap());
        },
        Err(e) => {
            panic!("{:?}", e);
        }
    }
}

// fn main() {
//     mod storage;
//     use storage::*;

//     use storage::models::*;
//     use diesel::prelude::*;
//     use storage::schema::storage::dsl::*;

//     let connection = establish_connection();
//     let results = storage
//         .select(StorageRecord::as_select())
//         .load(connection)
//         .expect("Error loading storage records");

//     println!("Storage:\n{:#?}", results);
// }



// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let addr = "[::1]:50051".parse()?;
//     let db_service = StorageService::default();

//     Server::builder()
//         .add_service(StorageServer::new(db_service))
//         .serve(addr)
//         .await?;

//     Ok(())
// }

