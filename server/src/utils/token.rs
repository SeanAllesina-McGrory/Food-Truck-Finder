use serde::Deserialize;
use std::borrow::Cow;
use serde::Serialize;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TokenStore {
    pub token_hash: Cow<'static, str>,
    pub expires: u64,
    pub vendor_id: Cow<'static, str>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Token {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u64,
}

#[derive(Debug, Deserialize)]
pub struct AuthorizeParams {
    pub code: String,
    pub state: String,
}

#[derive(Debug, Deserialize)]
pub struct Email {
    pub email: String,
    pub id: String,
}
