use chrono::{
    DateTime,
    Utc,
};
use log::debug;
use sqlx::{
    PgConnection,
    PgPool,
};

use crate::{
    db::RecordResult,
    snowflake::{
        SnowflakeGenerator,
        SnowflakeID,
    },
};

/// A database record of a bluesky post.
#[derive(Debug)]
pub struct PostRecord {
    /// The unique 64-bit [Snowflake] ID of the post.
    pub id: SnowflakeID,
    /// The unique CID of the post.
    pub cid: String,
    /// The ATProto URI of the post.
    pub uri: String,
    /// The Decentralized Identifier of the account that made the post.
    pub author_did: String,
    /// The time the account was created in the local database.
    pub created_at: DateTime<Utc>,
    /// The time the account was last updated in the local database.
    pub updated_at: DateTime<Utc>,
}

/// Information to insert or update a post record in the database.
pub struct PostInsert {
    /// The unique CID of the post.
    pub cid: String,
    /// The ATProto URI of the post.
    pub uri: String,
    /// The Decentralized Identifier of the account that made the post.
    pub author_did: String,
}

impl PostRecord {
    /// Insert a new post record into the database, or update an existing record if one already
    /// exists.
    pub async fn insert_or_update(
        db: &mut PgConnection,
        snowflake: &SnowflakeGenerator,
        insert: PostInsert,
    ) -> RecordResult<Self> {
        let id = snowflake.generate()?;
        debug!("Generated Snowflake ID: {} for {}", id, insert.cid);
        let record = sqlx::query_file_as_unchecked!(
            PostRecord,
            "sql/upsert/post.sql",
            SnowflakeID(id),
            insert.cid,
            insert.uri,
            insert.author_did
        )
        .fetch_one(db)
        .await?;

        Ok(record)
    }

    /// Retrieve a post record from the database using the account's unique 64-bit [Snowflake] ID.
    pub async fn find_by_snowflake(db: &PgPool, id: SnowflakeID) -> RecordResult<Option<Self>> {
        let record = sqlx::query_as_unchecked!(Self, "SELECT * FROM posts WHERE id = $1", id)
            .fetch_optional(db)
            .await?;

        Ok(record)
    }
}
