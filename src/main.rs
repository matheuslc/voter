use std::str::FromStr;

use tokio;
use tokio_postgres::{NoTls};

mod poll;
mod user;

use poll::Repository as pollRepository;
use user::Repository as userRepository;

use tonic::{transport::Server, Request, Response, Status};

use voterproto::vote_service_server::{VoteService, VoteServiceServer};
use voterproto::{CreateVoteRequest, CreateVoteResponse};

pub mod voterproto {
    include!("../proto/voterproto.rs");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Database connection string
    let conn_string = "host=localhost user=postgres password=pgpass dbname=tsdb";
    let config = tokio_postgres::Config::from_str(conn_string).unwrap();

    let manager = bb8_postgres::PostgresConnectionManager::new(config, NoTls);
    let pool = bb8::Pool::builder()
        .max_size(250)
        .build(manager)
        .await
        .unwrap();

    let repo = user::PGRepository { client: pool.clone() };
    let unreg_user = user::create_unregisted("mematheuslc@gmail.com".to_string());
    let final_user = repo.create(unreg_user.unwrap()).await;

    println!("User created: {:?}", final_user);

    let poll_repo = poll::PGRepository { client: pool.clone() };
    poll::UnregistedPoll { poll_name: "Test".to_string() };

    poll_repo.create_option(poll::UnregistedOption { option_name: "a".to_string(), option_order: 1 }).await?;
    poll_repo.create_option(poll::UnregistedOption { option_name: "b".to_string(), option_order: 2 }).await?;
    poll_repo.create_poll(poll::UnregistedPoll { poll_name: "Test".to_string() }).await?;

    let addr = "[::1]:50051".parse().unwrap();
    let s = VoterServiceReady{ poll_repo: poll_repo };

    println!("Voter server listening on {}", addr);

    Server::builder()
        .add_service(VoteServiceServer::new(s))
        .serve(addr)
        .await?;

    Ok(())
}

#[derive(Debug)]
pub struct VoterServiceReady {
    pub poll_repo: poll::PGRepository,
}

#[tonic::async_trait]
impl VoteService for VoterServiceReady {
    async fn vote(
        &self,
        request: Request<CreateVoteRequest>,
    ) -> Result<Response<CreateVoteResponse>, Status> {
        println!("Request from {:?}", request.remote_addr());
        println!("Created options for {:?}", request.remote_addr());

        let vot =  self.poll_repo.create_vote(poll::UnregistedVote { 
            poll_id: request.get_ref().poll_id,
            option_id: request.get_ref().option_id,
            user_id: request.get_ref().user_id, 
        }).await;

        let response = CreateVoteResponse { status: 0 };

        match vot {
            Ok(_) => println!(),
            Err(e) => println!("Error creating vote {:?}", e),
        }

        Ok(Response::new(response))
    }
}
