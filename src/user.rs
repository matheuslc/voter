use async_trait::async_trait;
use tokio_postgres::Error;
use tokio_postgres::Row;
use serde::Deserialize;
use validator::{Validate, ValidationErrors};
use bb8::{Pool};
use bb8_postgres::{PostgresConnectionManager};
use tokio_postgres::{NoTls};

#[derive(Debug, Clone, Deserialize)]
pub struct User {
    pub user_id: i32,
    pub email: String,
}

#[derive(Debug, Validate, Deserialize)]
pub struct UnregistedUser {
    #[validate(email)]
    email: String,
}

// ## Example
/// ```
/// let unreg_user = create_unregisted("test@test");
/// assert_eq!(unreg_user.email, "test@test");
/// ```
pub fn create_unregisted(email: String) -> Result<UnregistedUser, ValidationErrors> {
    let ur = UnregistedUser { email };

    match ur.validate() {
        Ok(_) => Ok(ur),
        Err(e) => Err(e),
    }
}

#[async_trait]
pub trait Repository {
    async fn create(&self, user: UnregistedUser) -> Result<User, Error>;
}

pub struct PGRepository {
    pub client: Pool<PostgresConnectionManager<NoTls>>,
}

#[async_trait]
impl Repository for PGRepository {
    async fn create(&self, user: UnregistedUser) -> Result<User, Error> {
        // Uses the client to create a new user in the database.
        let conn = self.client.get().await.unwrap();

        let statement = conn.prepare("INSERT INTO users (email) VALUES ($1) RETURNING user_id, email").await?;
        let row: Row = conn.query_one(&statement, &[&user.email]).await?;

        Ok(User {
            user_id: row.get(0),
            email: row.get(1),
        })
    }
}