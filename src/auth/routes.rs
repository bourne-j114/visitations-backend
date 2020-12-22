use actix_web::{get, post, web, HttpRequest, HttpResponse, Result};
use crate::constants;
use crate::users::{UsersMessage, Users, UserToken, UserTokenMessage, UserMessageInfo, UsersInfo};
use crate::utils::token_utils;
use crate::response::ResponseBody;


#[post("/signup")]
pub async fn signup(user: web::Json<UserMessageInfo>) -> Result<HttpResponse> {
    match Users::signup(UsersMessage {
        email: user.email.clone(),
        password: user.password.clone(),
    }) {
        Ok(message) => Ok(HttpResponse::Ok().json(ResponseBody::new(message.as_str(), &user.email))),
        Err(message) => Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_SIGNUP_FAILED, message)))
    }
}

#[post("/login")]
pub async fn login(user: web::Json<UserMessageInfo>) -> Result<HttpResponse> {
    match Users::login(UsersMessage {
        email: user.email.clone(),
        password: user.password.clone(),
    }) {
        Some(logged_user) => {
            let token_string = UserTokenMessage {
                token: UserToken::generate_token(logged_user),
                token_type: constants::BEARER.to_string(),
            };
            Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_LOGIN_SUCCESS, token_string)))
        }
        None => Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_LOGIN_FAILED, constants::EMPTY)))
    }
}

#[get("/verify")]
pub async fn verify(req: HttpRequest) -> Result<HttpResponse> {
    let authen_result = req.headers().get(constants::AUTHORIZATION);
    match authen_result {
        Some(authen_header) => {
            let authen_str = authen_header.to_str().unwrap();
            if authen_str.starts_with(constants::BEARER) {
                let token = authen_str[6..authen_str.len()].trim();
                // println!("{:?}", token);
                let token_data = token_utils::decode_token(token.to_string()).unwrap();
                let email = token_utils::verify_token(&token_data).unwrap();
                let logged_user = UsersInfo {
                    email,
                    login_session: token_data.claims.login_session,
                };
                let token_string = UserTokenMessage {
                    token: UserToken::generate_token(logged_user),
                    token_type: constants::BEARER.to_string(),
                };

                Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_VERIFY_TOKEN, token_string)))
            } else {
                Ok(HttpResponse::BadRequest().json(ResponseBody::new(constants::MESSAGE_TOKEN_MISSING, constants::EMPTY)))
            }
        }
        _ => Ok(HttpResponse::BadRequest().json(ResponseBody::new(constants::MESSAGE_TOKEN_MISSING, constants::EMPTY)))
    }
}

#[post("/logout")]
pub async fn logout(req: HttpRequest) -> Result<HttpResponse> {
    if let Some(authen_header) = req.headers().get(constants::AUTHORIZATION) {
        if let Ok(authen_str) = authen_header.to_str() {
            if authen_str.starts_with(constants::BEARER) {
                let token = authen_str[6..authen_str.len()].trim();
                if let Ok(token_data) = token_utils::decode_token(token.to_string()) {
                    if let Ok(email) = token_utils::verify_token(&token_data) {
                        if let Ok(user) = Users::find_by_email(email) {
                            Users::logout(user.id);
                        }
                    }
                }
            }
        };

        Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_LOGOUT_SUCCESS, constants::EMPTY)))
    } else {
        Ok(HttpResponse::BadRequest().json(ResponseBody::new(constants::MESSAGE_TOKEN_MISSING, constants::EMPTY)))
    }
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/auth")
            .service(signup)
            .service(login)
            .service(verify)
            .service(logout)
    );
}
