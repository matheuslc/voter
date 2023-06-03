use tokio_postgres::NoTls;
use tokio;

mod user;

use user::Repository;

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

    let repo = user::PGRepository { client };
    let unregUser = user::create_unregisted("mematheuslc@gmail.com".to_string());
    let user = repo.create(unregUser).await;

    match user {
        Ok(user) => println!("User created: {:?}", user),
        Err(e) => println!("Error creating user: {:?}", e),
    }
}
