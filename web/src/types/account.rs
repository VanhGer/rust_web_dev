use chrono::prelude::*;
use serde::{Deserialize, Serialize};
/// The Session object represents a session of token.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Session {
    /// Expiration time of the session.
    pub exp: DateTime<Utc>,
    // The ID of the account associated with the session.
    pub account_id: AccountId,
    /// "Not before" time of the session.
    pub nbf: DateTime<Utc>,
}
/// The Account object represents User account, including email, password and its id.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Account {
    /// Optional unique identifier of the account.
    pub id: Option<AccountId>,
    /// Email address associated with the account.
    pub email: String,
    /// Password of the account.
    pub password: String,
}

/// Represents the unique identifier for an account.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct AccountId(pub i32);