use tokio;
use tokio_postgres::NoTls;

mod poll;
mod user;

use poll::Repository as pollRepository;
use user::Repository as userRepository;

use tonic::{transport::Server, Request, Response, Status};

use voterproto::vote_service_server::{VoteService, VoteServiceServer};
use voterproto::{CreateVoteRequest, CreateVoteResponse};

pub mod voterproto {
    tonic::include_proto!("voterproto");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Database connection string
    // let conn_string = "host=localhost user=postgres password=pgpass dbname=tsdb";

    // Connect to the database
    // let (client, connection) = tokio_postgres::connect(conn_string, NoTls)
    //     .await
    //     .expect("Failed to connect to Postgres database");

    let addr = "[::1]:50051".parse().unwrap();
    let s = VoterServiceReady::default();

    println!("Voter server listening on {}", addr);

    Server::builder()
        .add_service(VoteServiceServer::new(s))
        .serve(addr)
        .await?;

    Ok(())
}

#[derive(Debug, Default)]
pub struct VoterServiceReady {}

#[tonic::async_trait]
impl VoteService for VoterServiceReady {
    async fn vote(
        &self,
        request: Request<voterproto::CreateVoteRequest>,
    ) -> Result<Response<voterproto::CreateVoteResponse>, Status> {
        println!("Request from {:?}", request.remote_addr());
        let response = voterproto::CreateVoteResponse { status: 0 };

        Ok(Response::new(response))
    }
}
