use atrium_api::app::bsky::actor::defs::{
    ProfileViewBasic,
    ProfileViewDetailed,
};
use mastodon_server_types::{
    account::{
        credentials::{
            CredentialAccount,
            Source,
        },
        role::Role,
        Account,
    },
    status::Visibility,
};
use sqlx::{
    PgConnection,
    PgPool,
};

use crate::{
    db::{
        accounts::{
            AccountDetailedInsert,
            AccountRecord,
            AccountSimpleInsert,
        },
        RecordError,
    },
    snowflake::SnowflakeGenerator,
};

pub async fn profile_basic_to_account(
    db: &mut PgConnection,
    snowflake: &SnowflakeGenerator,
    profile: &ProfileViewBasic,
) -> Result<Account, RecordError> {
    let record = AccountRecord::insert_or_update_simple(
        db,
        snowflake,
        AccountSimpleInsert {
            did: profile.did.clone(),
            avatar: profile.avatar.clone().unwrap_or_default(),
        },
    )
    .await?;

    Ok(Account {
        id: record.id.0.to_string(),
        username: profile.handle.clone(),
        acct: profile.handle.clone(),
        url: format!("https://bsky.social/{}", profile.handle),
        display_name: profile
            .display_name
            .clone()
            .unwrap_or_else(|| profile.handle.clone()),
        note: record.description,
        avatar: record.avatar.clone(),
        avatar_static: record.avatar,
        header: record.banner.clone(),
        header_static: record.banner,
        locked: false,
        fields: vec![],
        emojis: vec![],
        bot: false,
        group: false,
        discoverable: None,
        noindex: None,
        moved: Box::new(None),
        suspended: None,
        limited: None,
        // TODO: present somewhat correct created_at
        created_at: Default::default(),
        last_status_at: None,
        statuses_count: 0,
        followers_count: record.followers_count as u32,
        following_count: record.follows_count as u32,
    })
}

pub async fn profile_detailed_to_account(
    db: &PgPool,
    snowflake: &SnowflakeGenerator,
    profile: &ProfileViewDetailed,
) -> Result<Account, RecordError> {
    let record = AccountRecord::insert_or_update_detailed(
        db,
        snowflake,
        AccountDetailedInsert {
            did: profile.did.clone(),
            banner: profile.banner.clone().unwrap_or_default(),
            avatar: profile.avatar.clone().unwrap_or_default(),
            followers_count: profile.followers_count.unwrap_or(0),
            follows_count: profile.follows_count.unwrap_or(0),
            posts_count: profile.posts_count.unwrap_or(0),
            description: profile.description.clone().unwrap_or_default(),
        },
    )
    .await?;

    Ok(Account {
        id: record.id.0.to_string(),
        username: profile.handle.clone(),
        acct: profile.handle.clone(),
        url: format!("https://bsky.social/{}", profile.handle),
        display_name: profile
            .display_name
            .clone()
            .unwrap_or_else(|| profile.handle.clone()),
        note: record.description,
        avatar: record.avatar.clone(),
        avatar_static: record.avatar,
        header: record.banner.clone(),
        header_static: record.banner,
        locked: false,
        fields: vec![],
        emojis: vec![],
        bot: false,
        group: false,
        discoverable: None,
        noindex: None,
        moved: Box::new(None),
        suspended: None,
        limited: None,
        // TODO: present somewhat correct created_at
        created_at: Default::default(),
        last_status_at: None,
        statuses_count: 0,
        followers_count: record.followers_count as u32,
        following_count: record.follows_count as u32,
    })
}

pub async fn profile_detailed_to_cred_account(
    db: &PgPool,
    snowflake: &SnowflakeGenerator,
    profile: &ProfileViewDetailed,
) -> Result<CredentialAccount, RecordError> {
    Ok(CredentialAccount {
        account: profile_detailed_to_account(db, snowflake, profile).await?,
        source: Source {
            note: profile.description.clone().unwrap(),
            fields: vec![],
            privacy: Visibility::Public,
            sensitive: false,
            language: "en".to_string(),
            follow_requests_count: 0,
        },
        role: Role {
            id: 1,
            name: "User".to_string(),
            color: "".to_string(),
            permissions: 0,
            highlighted: false,
        },
    })
}
