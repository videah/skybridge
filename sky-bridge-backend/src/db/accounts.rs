use chrono::{
    DateTime,
    Utc,
};
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

/// A database record of a user's account.
#[derive(Debug)]
pub struct AccountRecord {
    /// The unique 64-bit [Snowflake] ID of the account.
    pub id: SnowflakeID,
    /// The unique Decentralized Identifier of the account.
    pub did: String,
    /// A URL string pointing to the account's avatar image.
    pub avatar: String,
    /// A URL string pointing to the account's banner image.
    pub banner: String,
    /// The number of followers the account has.
    pub followers_count: i32,
    /// The number of accounts the account follows.
    pub follows_count: i32,
    /// The number of posts the account has made.
    pub posts_count: i32,
    /// The bio or description of the account.
    pub description: String,
    /// The time the account was created in the local database.
    pub created_at: DateTime<Utc>,
    /// The time the account was last updated in the local database.
    pub updated_at: DateTime<Utc>,
}

/// Detailed information to insert or update an account record in the database.
/// Includes fields that don't exist in simplified views of the account's profile.
pub struct AccountDetailedInsert {
    /// The unique Decentralized Identifier of the account.
    pub did: String,
    /// A URL string pointing to the account's avatar image.
    pub avatar: String,
    /// A URL string pointing to the account's banner image.
    pub banner: String,
    /// The number of followers the account has.
    pub followers_count: i32,
    /// The number of accounts the account follows.
    pub follows_count: i32,
    /// The number of posts the account has made.
    pub posts_count: i32,
    /// The bio or description of the account.
    pub description: String,
}

/// Simplified information to insert or update an account record in the database.
pub struct AccountSimpleInsert {
    /// The unique Decentralized Identifier of the account.
    pub did: String,
    /// A URL string pointing to the account's avatar image.
    pub avatar: String,
}

impl AccountRecord {
    /// Insert a new account record into the database, or update an existing record if one already
    /// exists.
    pub async fn insert_or_update_simple(
        db: &mut PgConnection,
        snowflake: &SnowflakeGenerator,
        insert: AccountSimpleInsert,
    ) -> RecordResult<Self> {
        let id = snowflake.generate()?;
        let record = sqlx::query_file_as_unchecked!(
            Self,
            "sql/upsert/account_simple.sql",
            SnowflakeID(id),
            insert.did,
            insert.avatar,
        )
        .fetch_one(db)
        .await?;

        Ok(record)
    }

    /// Insert a new account record into the database, or update an existing record if one already
    /// exists.
    pub async fn insert_or_update_detailed(
        db: &PgPool,
        snowflake: &SnowflakeGenerator,
        insert: AccountDetailedInsert,
    ) -> RecordResult<Self> {
        let id = snowflake.generate()?;
        // SQLx currently has a very silly limitation when using custom types with query_as macros
        // as the list of bindable types is hardcoded, and you have to do some weird stuff to get
        // around it. For now, we have to give up type safety with the unchecked variant :(
        // https://github.com/launchbadge/sqlx/issues/339
        //
        // TODO: figure out a way to get type safety on these queries back
        let record = sqlx::query_file_as_unchecked!(
            Self,
            "sql/upsert/account_detailed.sql",
            SnowflakeID(id),
            insert.did,
            insert.banner,
            insert.avatar,
            insert.followers_count,
            insert.follows_count,
            insert.posts_count,
            insert.description
        )
        .fetch_one(db)
        .await?;

        Ok(record)
    }

    /// Retrieve an account record from the database using the account's unique 64-bit [Snowflake]
    /// ID.
    pub async fn find_by_snowflake(db: &PgPool, id: SnowflakeID) -> RecordResult<Option<Self>> {
        let record = sqlx::query_as_unchecked!(Self, "SELECT * FROM accounts WHERE id = $1", id)
            .fetch_optional(db)
            .await?;

        Ok(record)
    }
}
