use std::fmt::{
    Display,
    Formatter,
};

use data_encoding::{
    DecodeError,
    BASE64,
};
use ring::hmac;
use serde::{
    de::DeserializeOwned,
    Deserialize,
    Deserializer,
    Serialize,
};
use thiserror::Error;

/// Wrapper for an HMAC signed object string that can be converted back and forth from a string with
/// the format `[base64(data)|base64(signature)]`.
#[derive(Debug, Serialize)]
pub struct HmacSigned<T> {
    /// The object/struct that was signed.
    pub data: T,
    /// The HMAC signature of the object.
    pub signature: String,
}

/// Errors that can occur when working with decoding [HmacSigned] objects.
#[derive(Debug, Error)]
pub enum SignedDecodeError {
    #[error("invalid data format, expected `[base64(data)|base64(signature)]` and got `{0}`")]
    InvalidDataFormat(String),
    #[error("failed to decode data from base64")]
    InvalidDataEncoding(#[from] DecodeError),
    #[error("failed to deserialize data")]
    FailedToDeserializeData(#[from] serde_json::Error),
    #[error("failed to decode signature from base64")]
    InvalidSignatureEncoding,
}

impl<T: DeserializeOwned> HmacSigned<T> {
    /// Converts a signed and packed HMAC string into a [HmacSigned] struct.
    /// If the string does not match the expected format of `[base64(data)|base64(signature)]` or
    /// if the `data` is malformed, an error is returned.
    pub fn from_string(s: &String) -> Result<Self, SignedDecodeError> {
        let parts: Vec<&str> = s.split('|').collect();
        if parts.len() != 2 {
            return Err(SignedDecodeError::InvalidDataFormat(s.to_owned()));
        }

        let decoded_data = BASE64.decode(parts[0].as_bytes())?;
        let data = serde_json::from_slice::<T>(&decoded_data)?;

        // Check that the signature is valid base64.
        let signature = parts[1].to_string();
        BASE64
            .decode(signature.as_bytes())
            .map_err(|_| SignedDecodeError::InvalidSignatureEncoding)?;
        Ok(HmacSigned { data, signature })
    }
}

impl<'de, T: DeserializeOwned> Deserialize<'de> for HmacSigned<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Self::from_string(&s).map_err(serde::de::Error::custom)
    }
}

impl<T: Serialize> Display for HmacSigned<T> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let serialized = serde_json::to_string(&self.data).unwrap();
        write!(
            f,
            "{}|{}",
            BASE64.encode(serialized.as_bytes()),
            self.signature
        )
    }
}

/// Signs and packs an object into a [HmacSigned] struct using the provided HMAC key.
pub fn sign_and_pack<T: Serialize>(data: T, key: &hmac::Key) -> HmacSigned<T> {
    let serialized = serde_json::to_string(&data).unwrap();
    let tag = hmac::sign(key, serialized.as_bytes());

    HmacSigned {
        data,
        signature: BASE64.encode(tag.as_ref()),
    }
}
