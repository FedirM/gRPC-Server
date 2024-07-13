use proto::database::{GetHashRecordRequest, GetHashRecordResponse, HashDB};
use tonic::{transport::Server, Request, Response, Status};

mod proto {
    tonic::include_proto!("database");
}

#[derive(Debug, Default)]
struct HashDBService {}

#[tonic::async_trait]
impl HashDB for HashDBService {
    fn get_record(
        &self,
        request: Request<GetHashRecordRequest>,
    ) -> Result<Response<GetHashRecordResponse>, Status> {
        println!("Get new request: {:?}", request);

        let req = request.into_inner();
        let reply = GetHashRecordResponse {
            key: "kek",
            value: b"lol",
        };

        Ok(Response::new(reply));
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let server = HashDBService::default();

    Server::builder()
        .add_service(HashDBService::new(server))
        .serve(addr)
        .await?;

    Ok(())
}
