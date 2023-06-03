use tokio_postgres::NoTls;
use tokio;

mod user;
mod poll;

use user::Repository as userRepository;
use poll::Repository as pollRepository;

#[tokio::main]
async fn main() {
    // Database connection string
    let conn_string = "host=localhost user=postgres password=pgpass dbname=tsdb";

    // Connect to the database
    let (client, connection) =
        tokio_postgres::connect(conn_string, NoTls).await.expect("Failed to connect to Postgres database");

    // The connection object performs the actual communication with the database,
    // so spawn it off to run on its own.
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let repo = user::PGRepository { client: &client };
    let unreg_user = user::create_unregisted("mematheuslc@gmail.com".to_string());
    let final_user = repo.create(unreg_user).await;
    
    let poll_repo = poll::PGRepository { client: &client };
    let first_poll = poll_repo.create_poll(poll::UnregistedPoll { poll_name: "First poll".to_string() }).await;
    let first_option = poll_repo.create_option(poll::UnregistedOption { option_name: "First option".to_string(), option_order: 1 }).await;
    let _second_option = poll_repo.create_option(poll::UnregistedOption { option_name: "Second option".to_string(), option_order: 2 }).await;


    let vote = poll_repo.create_vote(poll::UnregistedVote { poll_id: first_poll.unwrap().poll_id, option_id: first_option.unwrap().option_id, user_id: final_user.unwrap().user_id }).await;

    match vote {
        Ok(v) => println!("Vote: {:?}", v),
        Err(e) => println!("Error: {:?}", e),
    }
}
