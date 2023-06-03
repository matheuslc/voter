use async_trait::async_trait;
use tokio_postgres::Error;
use tokio_postgres::Row;

#[derive(Debug)]
pub struct User {
    pub user_id: i32,
    pub email: String,
}

pub struct UnregistedUser {
    email: String,
}

pub fn create_unregisted(email: String) -> UnregistedUser {
    UnregistedUser { email }
}

#[async_trait]
pub trait Repository {
    async fn create(&self, user: UnregistedUser) -> Result<User, Error>;
}

pub struct PGRepository {
    pub client: tokio_postgres::Client,
}

#[async_trait]
impl Repository for PGRepository {
    async fn create(&self, user: UnregistedUser) -> Result<User, Error> {
        // Uses the client to create a new user in the database.

        let statement = self
            .client
            .prepare("INSERT INTO users (email) VALUES ($1) RETURNING user_id, email")
            .await?;
        let row: Row = self.client.query_one(&statement, &[&user.email]).await?;

        Ok(User {
            user_id: row.get(0),
            email: row.get(1),
        })
    }
}