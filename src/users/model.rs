use chrono::{NaiveDateTime, Local};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use bcrypt::{hash, verify, DEFAULT_COST};
use crate::api_error::ApiError;
use crate::{db, constants};
use crate::schema::users;
use crate::users::UserToken;

#[derive(Serialize, Deserialize, AsChangeset)]
#[table_name = "users"]
pub struct UsersMessage {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, AsChangeset)]
#[table_name = "users"]
pub struct UsersInfo {
    pub email: String,
    pub login_session: String,
}

#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "users"]
pub struct Users {
    pub id: Uuid,
    pub email: String,
    pub password: String,
    pub login_session: String,
    #[serde(skip_serializing)]
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

impl Users {
    pub fn signup(user: UsersMessage) -> Result<String, String> {

        if Self::find_by_email(user.email.clone()).is_err() {
            let hashed_pwd = hash(&user.password.clone(), DEFAULT_COST).unwrap();
            let mut insert_user = Users::from(user);
            insert_user.password = hashed_pwd;

            let conn = db::connection().unwrap();
            diesel::insert_into(users::table)
                .values(&insert_user)
                .execute(&conn).unwrap();

            Ok(constants::MESSAGE_SIGNUP_SUCCESS.to_string())
        } else {
            Err(format!("User '{}' is already registered", &user.email))
        }
    }

    pub fn find_all() -> Result<Vec<Self>, ApiError> {
        let conn = db::connection().unwrap();

        let users = users::table
            .load::<Users>(&conn)?;

        Ok(users)
    }

    pub fn find(id: Uuid) -> Result<Self, ApiError> {
        let conn = db::connection()?;

        let user = users::table
            .filter(users::id.eq(id))
            .first(&conn)?;

        Ok(user)
    }

    pub fn find_by_email(email: String) -> Result<Self, ApiError> {
        let conn = db::connection()?;

        let user = users::table
            .filter(users::email.eq(email))
            .first(&conn)?;

        Ok(user)
    }

    pub fn create(user: UsersMessage) -> Result<Self, ApiError> {
        let conn = db::connection()?;

        let mut user = Users::from(user);
        user.hash_password()?;
        let user = diesel::insert_into(users::table)
            .values(user)
            .get_result(&conn)?;

        Ok(user)
    }

    pub fn update(id: Uuid, user: UsersMessage) -> Result<Self, ApiError> {
        let conn = db::connection()?;

        let user = diesel::update(users::table)
            .filter(users::id.eq(id))
            .set(user)
            .get_result(&conn)?;

        Ok(user)
    }

    pub fn delete(id: Uuid) -> Result<usize, ApiError> {
        let conn = db::connection()?;

        let res = diesel::delete(
            users::table
                .filter(users::id.eq(id))
        )
            .execute(&conn)?;

        Ok(res)
    }

    pub fn hash_password(&mut self) -> Result<(), ApiError> {
        // let salt: [u8; 32] = rand::thread_rng().gen();
        // let config = Config::default();
        // self.password = argon2::hash_encoded(self.password.as_bytes(), &salt, &config)
        //     .map_err(|e| ApiError::new(500, format!("Failed to hash password: {}", e)))?;

        self.password = hash(&self.password.clone(), DEFAULT_COST).unwrap();

        Ok(())
    }

    pub fn generate_login_session() -> String {
        Uuid::new_v4().simple().to_string()
    }

    // pub fn verify_password(&self, password: &[u8]) -> Result<bool, ApiError> {
    //     argon2::verify_encoded(&self.password, password)
    //         .map_err(|e| ApiError::new(500, format!("Failed to verify password: {}", e)))
    // }

    pub fn login(user: UsersMessage) -> Option<UsersInfo> {
        let conn = db::connection().unwrap();

        let user_to_verify = users::table
            .filter(users::email.eq(&user.email))
            .get_result::<Users>(&conn);

        match user_to_verify {
            Ok(user_found) => {
                if !user_found.password.is_empty()
                    && verify(&user.password, &user_found.password).unwrap()
                {
                    let login_session_str = Users::generate_login_session();
                    if Users::update_login_session(&user_found.email, &login_session_str) {
                        return Some(UsersInfo {
                            email: user_found.email,
                            login_session: login_session_str,
                        });
                    }
                }
            },
            _ => return Option::None
        }

        None
    }

    pub fn update_login_session(email: &str, login_session_str: &str) -> bool {
        let conn = db::connection().unwrap();
        if let Ok(user) = Users::find_by_email(email.to_string()) {
            diesel::update(users::table)
                .filter(users::id.eq(user.id))
                .set(users::login_session.eq(login_session_str.to_string()))
                .execute(&conn)
                .is_ok()
        } else {
            false
        }
    }

    pub fn logout(user_id: Uuid) {
        let conn = db::connection().unwrap();
        if let Ok(user) = users::table
            .filter(users::id.eq(user_id))
            .get_result::<Users>(&conn) {
                Self::update_login_session(&user.email, "");
            }
    }

    pub fn is_valid_login_session(user_token: &UserToken) -> bool {
        let conn = db::connection().unwrap();
        users::table
            .filter(users::email.eq(&user_token.user))
            .filter(users::login_session.eq(&user_token.login_session))
            .get_result::<Users>(&conn)
            .is_ok()
    }
}

impl From<UsersMessage> for Users {
    fn from(users_message: UsersMessage) -> Self {
        Users {
            id: Uuid::new_v4(),
            email: users_message.email,
            password: users_message.password,
            login_session: "".to_string(),
            created_at: Local::now().naive_local(),
            updated_at: None,
        }
    }
}
