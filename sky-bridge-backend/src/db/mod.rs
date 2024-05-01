use thiserror::Error;

pub mod accounts;
pub mod posts;
pub mod session;

pub type RecordResult<T> = Result<T, RecordError>;

/// An error that can occur when working with records in the database.
#[derive(Debug, Error)]
pub enum RecordError {
    #[error("database error: {0}")]
    DatabaseFailure(#[from] sqlx::Error),
    #[error("snowflake generation failure: {0}")]
    SnowflakeFailure(#[from] snowdon::Error),
    #[error("unexpected record type")]
    UnexpectedRecordType,
}
