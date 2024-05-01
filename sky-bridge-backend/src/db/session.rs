use async_trait::async_trait;
use atrium_api::agent::{
    store::SessionStore,
    Session,
};
use log::debug;
use sqlx::{
    types::chrono::{
        DateTime,
        Utc,
    },
    PgPool,
};

/// A database record of a user's bluesky session.
pub struct SessionRecord {
    /// The database ID of the session.
    pub id: i32,
    /// The Decentralized Identifier of the user that owns the session.
    pub did: String,
    /// The handle of the user that owns the session.
    pub handle: String,
    /// The access token for the session.
    pub access_jwt: String,
    /// The refresh token for the session.
    pub refresh_jwt: String,
    /// The time the session was created.
    pub created_at: DateTime<Utc>,
    /// The time the session was last updated.
    pub updated_at: DateTime<Utc>,
}

/// A new session record to be inserted into the database.
pub struct SessionInsert {
    /// The Decentralized Identifier of the user that owns the session.
    pub did: String,
    /// The handle of the user that owns the session.
    pub handle: String,
    /// The access token for the session.
    pub access_jwt: String,
    /// The refresh token for the session.
    pub refresh_jwt: String,
}

impl SessionRecord {
    /// Insert a new session record into the database, or update an existing record if one already
    /// exists.
    async fn insert_or_update(db: &PgPool, insert: SessionInsert) -> sqlx::Result<SessionRecord> {
        sqlx::query_file_as!(
            SessionRecord,
            "sql/upsert/session.sql",
            insert.did,
            insert.handle,
            insert.access_jwt,
            insert.refresh_jwt
        )
        .fetch_one(db)
        .await
    }

    pub async fn delete(db: &PgPool, did: &str) -> sqlx::Result<()> {
        sqlx::query_file!("sql/delete_session.sql", did)
            .execute(db)
            .await?;
        Ok(())
    }

    /// Retrieve a session record from the database using the user's Decentralized Identifier.
    pub async fn get_from_did(db: &PgPool, did: &str) -> sqlx::Result<Option<SessionRecord>> {
        sqlx::query_file_as!(SessionRecord, "sql/get_session_from_did.sql", did)
            .fetch_optional(db)
            .await
    }
}

pub struct DatabaseSessionStore {
    did: String,
    db: PgPool,
}

impl DatabaseSessionStore {
    pub fn new(did: String, db: PgPool) -> Self {
        Self { did, db }
    }
}

#[async_trait]
impl SessionStore for DatabaseSessionStore {
    async fn get_session(&self) -> Option<Session> {
        debug!("Retrieving session for DID: {:?}", self.did);
        let record = SessionRecord::get_from_did(&self.db, &self.did)
            .await
            .expect("database error retrieving session");

        match record {
            Some(record) => Some(Session {
                access_jwt: record.access_jwt,
                did: record.did,
                did_doc: None,
                email: None,
                email_confirmed: None,
                handle: record.handle,
                refresh_jwt: record.refresh_jwt,
            }),
            None => None,
        }
    }
    async fn set_session(&self, session: Session) {
        debug!("Setting session for DID: {:?}", self.did);
        SessionRecord::insert_or_update(
            &self.db,
            SessionInsert {
                did: session.did,
                handle: session.handle,
                access_jwt: session.access_jwt,
                refresh_jwt: session.refresh_jwt,
            },
        )
        .await
        .expect("database error updating session");
    }
    async fn clear_session(&self) {
        debug!("Clearing session for DID: {:?}", self.did);
        SessionRecord::delete(&self.db, &self.did)
            .await
            .expect("database error deleting session");
    }
}
