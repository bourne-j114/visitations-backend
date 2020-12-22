use crate::api_error::ApiError;
use crate::users::{Users, UsersMessage};
use actix_web::{delete, get, post, put, web, HttpResponse};
use serde_json::json;
use uuid::Uuid;

#[get("/list")]
async fn users_list_all() -> Result<HttpResponse, ApiError> {
    let users = Users::find_all()?;
    Ok(HttpResponse::Ok().json(users))
}

#[post("/create")]
async fn users_create(user: web::Json<UsersMessage>) -> Result<HttpResponse, ApiError> {
    let user = Users::create(user.into_inner())?;
    Ok(HttpResponse::Ok().json(user))
}

#[get("/get/{id}")]
async fn users_get(id: web::Path<Uuid>) -> Result<HttpResponse, ApiError> {
    let user = Users::find(id.into_inner())?;
    Ok(HttpResponse::Ok().json(user))
}


#[put("/update/{id}")]
async fn users_update(id: web::Path<Uuid>, user: web::Json<UsersMessage>) -> Result<HttpResponse, ApiError> {
    let user = Users::update(id.into_inner(), user.into_inner())?;
    Ok(HttpResponse::Ok().json(user))
}

#[delete("/delete/{id}")]
async fn users_delete(id: web::Path<Uuid>) -> Result<HttpResponse, ApiError> {
    let num_deleted = Users::delete(id.into_inner())?;
    Ok(HttpResponse::Ok().json(json!({ "deleted": num_deleted })))
}

#[allow(dead_code)]
pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/users")
            .service(users_list_all)
            .service(users_get)
            .service(users_create)
            .service(users_update)
            .service(users_delete)
    );
}
