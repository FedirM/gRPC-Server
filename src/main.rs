use proto::{
    database_server::{Database, DatabaseServer},
    kektor_server::{Kektor, KektorServer},
    GetRecordRequest, GetRecordResponse, KektorRequest, KektorResponse,
};
use tokio::runtime::Runtime;
use tonic::{transport::Server, Request, Response, Status};

mod proto {
    tonic::include_proto!("database");
}

#[derive(Debug, Default)]
struct DatabaseService {}

#[tonic::async_trait]
impl Database for DatabaseService {
    async fn get_record(
        &self,
        request: Request<GetRecordRequest>,
    ) -> Result<Response<GetRecordResponse>, Status> {
        println!("Get new request: {:#?}", request);

        let req = request.into_inner();
        println!("Inner object: {:#?}", req);

        let reply = GetRecordResponse {
            key: "kek".to_string(),
            value: b"lol".to_vec(),
        };

        Ok(Response::new(reply))
    }
}

#[derive(Debug, Default)]
struct KektorService {}

#[tonic::async_trait]
impl Kektor for KektorService {
    async fn kek(
        &self,
        request: Request<KektorRequest>,
    ) -> Result<Response<KektorResponse>, Status> {
        let req = request.into_inner();
        println!("Kektor req: {:#?}", req);

        Ok(Response::new(KektorResponse {
            val: "Hello, from kektor!".to_string(),
        }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr1 = "[::1]:50051".parse()?;
    let addr2 = "[::1]:50052".parse()?;
    let db_service = DatabaseService::default();
    let kek_service = KektorService::default();

    let th1 = std::thread::spawn(move || {
        let rt1 = Runtime::new().unwrap();
        rt1.block_on(async move {
            Server::builder()
                .add_service(DatabaseServer::new(db_service))
                .serve(addr1)
                .await
                .unwrap()
        });
    });

    let th2 = std::thread::spawn(move || {
        let rt2 = Runtime::new().unwrap();
        rt2.block_on(async move {
            Server::builder()
                .add_service(KektorServer::new(kek_service))
                .serve(addr2)
                .await
                .unwrap()
        });
    });

    th1.join().unwrap();
    th2.join().unwrap();

    Ok(())
}

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let addr = "[::1]:50051".parse()?;
//     let service = DatabaseService::default();

//     Server::builder()
//         .add_service(DatabaseServer::new(service))
//         .serve(addr)
//         .await?;

//     Ok(())
// }
