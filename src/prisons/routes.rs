use crate::api_error::ApiError;

use actix_web::{delete, get, post, put, web, HttpResponse};
use serde_json::json;
use crate::prisons::{Prisons, PrisonsMessage, PrisonLocation, PrisonLocationMessage, import_family_and_friends};
use crate::visitors::Visitors;
use crate::visits::Visits;
use std::error::Error;
use actix_multipart::Multipart;
use std::borrow::BorrowMut;
use crate::utils::upload::{save_file as upload_save_file, split_payload, UploadFile};
use serde::{Deserialize, Serialize};
use crate::cases::{CasesMessage, Cases};

#[derive(Deserialize, Serialize, Debug)]
pub struct PrisonInfo {
    pub profile: PrisonsMessage,
    pub case_datail: CasesMessage,
}


#[get("/list")]
async fn prisons_list_all() -> Result<HttpResponse, ApiError> {
    let prisons = Prisons::find_all()?;
    Ok(HttpResponse::Ok().json(prisons))
}
/*
#[post("/create")]
async fn prisons_create(user: web::Json<PrisonsMessage>) -> Result<HttpResponse, ApiError> {
    let user = Prisons::create(user.into_inner())?;
    Ok(HttpResponse::Ok().json(user))
}
*/
#[post("/register/{id}")]
async fn register(params: web::Path<String>, mut payload: Multipart) -> Result<HttpResponse, ApiError> {
    let prison_id = params.into_inner();
    let pl = split_payload(payload.borrow_mut(), prison_id.clone()).await;
    println!("bytes={:#?}", pl.0);
    let prison_info: PrisonInfo = serde_json::from_slice(&pl.0).unwrap();
    println!("converter_struct={:#?}", prison_info);
    let prison = Prisons::create(prison_info.profile, prison_id)?;
    let cases = Cases::insert(prison_info.case_datail)?;
    Ok(HttpResponse::Ok().json(prison))
}

#[get("/import")]
async fn import() -> Result<HttpResponse, ApiError> {
    import_family_and_friends().await.unwrap();
    Ok(HttpResponse::Ok().json(json!({ "OK": "1" })))
}

#[get("/get/{id}")]
async fn prisons_get(params: web::Path<String>) -> Result<HttpResponse, ApiError> {
    let prison = Prisons::find(params.into_inner())?;
    let mut visitors = vec![];
    let mut visits = vec![];
    if prison.first_name != "" {
        visitors = Visitors::find_family_and_friend(prison.prison_id.clone())?;
        visits = Visits::find_by_prison_id(prison.prison_id.clone())?;
    }
    Ok(HttpResponse::Ok().json(json!({ "prison": prison,"visitors": visitors ,"visits":visits})))
}


#[post("/update-location")]
async fn prisons_update_location(params: web::Json<PrisonLocationMessage>) -> Result<HttpResponse, ApiError> {
    let user = PrisonLocation::update(params.into_inner())?;
    Ok(HttpResponse::Ok().json(user))
}

#[delete("/delete/{id}")]
async fn prisons_delete(id: web::Path<String>) -> Result<HttpResponse, ApiError> {
    let num_deleted = Prisons::delete(id.into_inner())?;
    Ok(HttpResponse::Ok().json(json!({ "deleted": num_deleted })))
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/prisons")
            .service(prisons_list_all)
            .service(prisons_get)
         //   .service(prisons_create)
            .service(prisons_update_location)
            .service(prisons_delete)
            .service(import)
            .service(register)
    );
}
