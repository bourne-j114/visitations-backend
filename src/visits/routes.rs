use crate::api_error::ApiError;

use actix_web::{delete, get, post, put, web, HttpResponse};
use serde_json::json;
use crate::visitors::{Visitors, VisitorsMessage};

#[get("/list")]
async fn visits_list_all() -> Result<HttpResponse, ApiError> {
    let users = Visitors::find_all()?;
    Ok(HttpResponse::Ok().json(users))
}

#[post("/create")]
async fn visits_create(user: web::Json<VisitorsMessage>) -> Result<HttpResponse, ApiError> {
    let user = Visitors::create(user.into_inner())?;
    Ok(HttpResponse::Ok().json(user))
}

#[get("/get/{id}")]
async fn visits_get(params: web::Path<i32>) -> Result<HttpResponse, ApiError> {
    let user = Visitors::find(params.into_inner())?;
    Ok(HttpResponse::Ok().json(user))
}


#[put("/update/{id}")]
async fn visits_update(id: web::Path<i32>, params: web::Json<VisitorsMessage>) -> Result<HttpResponse, ApiError> {
    let user = Visitors::update(id.into_inner(), params.into_inner())?;
    Ok(HttpResponse::Ok().json(user))
}

#[delete("/delete/{id}")]
async fn visits_delete(id: web::Path<i32>) -> Result<HttpResponse, ApiError> {
    let num_deleted = Visitors::delete(id.into_inner())?;
    Ok(HttpResponse::Ok().json(json!({ "deleted": num_deleted })))
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/visitors")
            .service(visits_list_all)
            .service(visits_get)
            .service(visits_create)
            .service(visits_update)
            .service(visits_delete)
    );
}
