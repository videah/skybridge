//! Mastodon style snowflake ID's

use serde::Deserialize;
use snowdon::{
    Epoch,
    Generator,
    Layout,
};
use sqlx::{
    encode::IsNull,
    error::BoxDynError,
    postgres::{
        PgArgumentBuffer,
        PgTypeInfo,
        PgValueRef,
    },
    Decode,
    Encode,
    Postgres,
};

/// The parameters for a snowflake ID.
#[derive(Debug)]
pub struct SnowflakeParams;

/// The epoch for a snowflake ID, when to start counting from.
#[derive(Debug)]
pub struct SnowflakeEpoch;

/// A generator for snowflake ID's.
pub type SnowflakeGenerator = Generator<SnowflakeParams, SnowflakeEpoch>;

/// A 64-bit snowflake ID.
pub type Snowflake = snowdon::Snowflake<SnowflakeParams, SnowflakeEpoch>;

/// A 64-bit snowflake ID newtype. This provides various traits for working with the ID with the
/// database.
#[derive(Debug, Deserialize)]
pub struct SnowflakeID(pub Snowflake);

impl SnowflakeParams {
    const TIMESTAMP_BITS: usize = 48;
    const SEQUENCE_NUMBER_BITS: usize = 16;
    const SEQUENCE_NUMBER_MASK: u64 = Self::SEQUENCE_NUMBER_BITS as u64;
    const TIMESTAMP_MASK: u64 = (Self::TIMESTAMP_BITS << Self::SEQUENCE_NUMBER_BITS) as u64;
}

impl Layout for SnowflakeParams {
    fn construct_snowflake(timestamp: u64, sequence_number: u64) -> u64 {
        assert!(
            !Self::exceeds_timestamp(timestamp) && !Self::exceeds_sequence_number(sequence_number)
        );
        timestamp << Self::SEQUENCE_NUMBER_BITS | sequence_number
    }
    fn timestamp(input: u64) -> u64 {
        (input & Self::TIMESTAMP_MASK) >> Self::SEQUENCE_NUMBER_BITS
    }
    fn exceeds_timestamp(input: u64) -> bool {
        input >= 1 << Self::TIMESTAMP_BITS
    }
    fn sequence_number(input: u64) -> u64 {
        input & Self::SEQUENCE_NUMBER_MASK
    }
    fn exceeds_sequence_number(input: u64) -> bool {
        input >= 1 << Self::SEQUENCE_NUMBER_BITS
    }
    fn is_valid_snowflake(_input: u64) -> bool {
        true
    }
}

impl Epoch for SnowflakeEpoch {
    fn millis_since_unix() -> u64 {
        1420070400000
    }
}

impl<'q> Encode<'q, Postgres> for SnowflakeID {
    fn encode_by_ref(&self, buf: &mut PgArgumentBuffer) -> IsNull {
        <i64 as Encode<Postgres>>::encode(self.0.into_inner() as i64, buf)
    }
}

impl<'r> Decode<'r, Postgres> for SnowflakeID {
    fn decode(value: PgValueRef<'r>) -> Result<Self, BoxDynError> {
        let inner = <i64 as Decode<Postgres>>::decode(value)?;
        Ok(SnowflakeID(Snowflake::from_raw(inner as u64)?))
    }
}

impl sqlx::Type<Postgres> for SnowflakeID {
    fn type_info() -> PgTypeInfo {
        <i64 as sqlx::Type<Postgres>>::type_info()
    }
    fn compatible(ty: &PgTypeInfo) -> bool {
        <i64 as sqlx::Type<Postgres>>::compatible(ty)
    }
}
