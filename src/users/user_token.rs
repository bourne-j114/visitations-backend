use jsonwebtoken::{EncodingKey, Header};
use serde_derive::{Deserialize, Serialize};
use crate::users::UsersInfo;
use std::str;
use crate::constants::KEY;
use chrono::Local;

static ONE_WEEK: i64 = 60 * 60 * 24 * 7; // in seconds

#[derive(Serialize, Deserialize)]
pub struct UserToken {
    // issued at
    pub iat: i64,
    // expiration
    pub exp: i64,
    // data
    pub user: String,
    pub login_session: String,
}

impl UserToken {
    pub fn generate_token(user: UsersInfo) -> String {
        let now = Local::now().timestamp_nanos() / 1_000_000_000; // nanosecond -> second
        let payload = UserToken {
            iat: now,
            exp: now + ONE_WEEK,
            user: user.email,
            login_session: user.login_session,
        };

        jsonwebtoken::encode(&Header::default(), &payload, &EncodingKey::from_secret(&KEY)).unwrap()
    }
}

#[derive(Serialize, Deserialize)]
pub struct UserTokenMessage {
    pub token: String,
    pub token_type: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserMessageInfo {
    pub email: String,
    pub password: String,
}
