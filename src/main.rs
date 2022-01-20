// Global crates
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

// Modules
mod actors;
mod db_utils;
mod models;
mod queries;
mod schema;
mod schema_graphql;
mod handlers;

// Libary imports
use actix_web::{App, HttpServer, web, HttpResponse};
use db_utils::{get_pool, run_migrations};
use dotenv::dotenv;
use schema_graphql::*;
use std::env;
use handlers::graphql::{ graphql, playground };

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server");
    // Start dotenv
    dotenv().ok();

    // Get database pool
    let db_url =
        env::var("DATABASE_URL").expect("Error retrieving the database url. Check your .env file");
    run_migrations(&db_url);
    let db_pool = get_pool(&db_url);

    // create Juniper schema
    let schema = std::sync::Arc::new(create_schema());

    HttpServer::new(move || {
        App::new()
            .service(
                web::resource("/graphql")
                    .route(web::get().to(graphql))
                    .route(web::post().to(graphql)),
            )
            .service(web::resource("/playground").route(web::get().to(playground)))
            .default_service(web::route().to(|| {
                HttpResponse::Found()
                    .header("location", "/playground")
                    .finish()
            }))
            .data(db_pool.clone())
            .data(schema.clone())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
