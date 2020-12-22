
// Static Variable
pub static KEY: [u8; 32] = *include_bytes!("../.secret.key");

// Messages
// pub const MESSAGE_OK: &str = "ok";
pub const MESSAGE_SIGNUP_SUCCESS: &str = "Signup successfully";
pub const MESSAGE_SIGNUP_FAILED: &str = "Error while signing up, please try again";
pub const MESSAGE_LOGIN_SUCCESS: &str = "Login successfully";
pub const MESSAGE_LOGIN_FAILED: &str = "Wrong username or password, please try again";
pub const MESSAGE_LOGOUT_SUCCESS: &str = "Logout successfully";
pub const MESSAGE_VERIFY_TOKEN: &str = "Verify token success";
pub const MESSAGE_INVALID_TOKEN: &str = "Invalid token, please login again";

// Bad request messages
pub const MESSAGE_TOKEN_MISSING: &str = "Token is missing";

// Headers
pub const AUTHORIZATION: &str = "Authorization";
pub const BEARER: &str = "Bearer";

// Misc
pub const EMPTY: &str = "";

// ignore routes
pub const IGNORE_ROUTES: [&str; 1] = [
    "/api",
];
