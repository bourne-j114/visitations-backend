use jsonwebtoken::{DecodingKey, TokenData, Validation};
use crate::users::{UserToken, Users};
use crate::constants::KEY;

pub fn decode_token(token: String) -> jsonwebtoken::errors::Result<TokenData<UserToken>> {
    jsonwebtoken::decode::<UserToken>(&token, &DecodingKey::from_secret(&KEY), &Validation::default())
}

pub fn verify_token(token_data: &TokenData<UserToken>) -> Result<String, String> {
    if Users::is_valid_login_session(&token_data.claims) {
        Ok(token_data.claims.user.to_string())
    } else {
        Err("Invalid token".to_string())
    }
}
