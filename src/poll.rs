use async_trait::async_trait;
use tokio_postgres::Error;
use tokio_postgres::Row;

#[derive(Debug)]
pub struct Poll {
    pub poll_id: i32,
    pub poll_name: String,
}

pub struct UnregistedPoll {
    pub poll_name: String,
}

#[derive(Debug)]
pub struct Option {
    pub option_id: i32,
    pub option_name: String,
    pub option_order: i32,
}

pub struct UnregistedOption {
    pub option_name: String,
    pub option_order: i32,
}

#[derive(Debug)]
pub struct Vote {
    pub vote_id: i32,
    pub option_id: i32,
    pub poll_id: i32,
    pub user_id: i32,
}

pub struct UnregistedVote {
    pub poll_id: i32,
    pub option_id: i32,
    pub user_id: i32,
}

#[async_trait]
pub trait Repository {
    async fn create_poll(&self, poll: UnregistedPoll) -> Result<Poll, Error>;
    async fn create_option(&self, option: UnregistedOption) -> Result<Option, Error>;
    async fn create_vote(&self, vote: UnregistedVote) -> Result<Vote, Error>;
}

pub struct PGRepository<'a> {
    pub client: &'a tokio_postgres::Client,
}

#[async_trait]
impl Repository for PGRepository<'_> {
    async fn create_poll(&self, poll: UnregistedPoll) -> Result<Poll, Error> {
        // Uses the client to create a new user in the database.

        let statement = self
            .client
            .prepare("INSERT INTO poll (poll_name) VALUES ($1) RETURNING poll_id, poll_name")
            .await?;
        let row: Row = self
            .client
            .query_one(&statement, &[&poll.poll_name])
            .await?;

        Ok(Poll {
            poll_id: row.get(0),
            poll_name: row.get(1)
        })
    }

    async fn create_option(&self, option: UnregistedOption) -> Result<Option, Error> {
        // Uses the client to create a new user in the database.

        let statement = self
            .client
            .prepare("INSERT INTO poll_options (option_name, option_order) VALUES ($1, $2) RETURNING option_id, option_name, option_order")
            .await?;

        let row: Row = self
            .client
            .query_one(
                &statement,
                &[&option.option_name, &option.option_order],
            )
            .await?;

        Ok(Option {
            option_id: row.get(0),
            option_name: row.get(1),
            option_order: row.get(2),
        })
    }

    async fn create_vote(&self, vote: UnregistedVote) -> Result<Vote, Error> {
        // Uses the client to create a new user in the database.

        let statement = self
            .client
            .prepare("INSERT INTO vote (poll_id, option_id, user_id, created_at) VALUES ($1, $2, $3, now()) RETURNING vote_id, option_id, user_id, poll_id")
            .await?;
        let row: Row = self
            .client
            .query_one(&statement, &[&vote.poll_id, &vote.option_id, &vote.user_id])
            .await?;

        Ok(Vote {
            vote_id: row.get(0),
            option_id: row.get(1),
            user_id: row.get(2),
            poll_id: row.get(3),
        })
    }
}
