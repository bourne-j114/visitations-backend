#[macro_use]
extern crate log;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

use actix_web::{App, HttpServer, http};
use actix_cors::Cors;
use std::env;
use dotenv::dotenv;

mod api_error;
mod auth;
mod constants;
mod db;
mod middleware;
mod response;
mod visitors;
mod schema;
mod utils;
mod users;
mod prisons;
mod visits;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();
    db::init();

    info!("Starting server");
    let host = env::var("HOST").expect("Host not set");
    let port = env::var("PORT").expect("Port not set");

    HttpServer::new(move||
        App::new()
            .wrap(Cors::default() // allowed_origin return access-control-allow-origin: * by default
                .allow_any_origin()
                .allowed_methods(vec!["GET", "POST", "OPTIONS", "PUT", "DELETE", "PATCH"])
                .allowed_headers(vec![
                    http::header::ORIGIN,
                    http::header::AUTHORIZATION,
                    http::header::ACCEPT,
                    http::header::ACCESS_CONTROL_ALLOW_HEADERS,
                    http::header::ACCESS_CONTROL_ALLOW_ORIGIN,
                    http::header::ACCESS_CONTROL_ALLOW_METHODS,
                    http::header::ACCESS_CONTROL_ALLOW_CREDENTIALS,
                ])
                .allowed_header(http::header::CONTENT_TYPE)
                .supports_credentials()
                .max_age(3600)
            )
            .wrap(actix_web::middleware::Logger::default())
           // .wrap(crate::middleware::authen_middleware::Authentication)
            .configure(auth::init_routes)
            .configure(users::init_routes)
            .configure(visitors::init_routes)
            .configure(prisons::init_routes)

    ).bind(format!("{}:{}", host, port))?
        .run().await
}
