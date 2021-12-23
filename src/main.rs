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

// Libary imports
use actix::SyncArbiter;
use actix_web::{App, HttpServer};
use actors::db::DBActor;
use db_utils::{get_pool, run_migrations};
use dotenv::dotenv;
use models::AppState;
use queries::*;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let db_url =
        env::var("DATABASE_URL").expect("Error retrieving the database url. Check your .env file");
    run_migrations(&db_url);
    let pool = get_pool(&db_url);
    let db_addr = SyncArbiter::start(5, move || DBActor(pool.clone()));

    HttpServer::new(move || {
        App::new()
            .service(health)
            .service(get_all)
            .service(get_published)
            .service(create_article)
            .service(publish_article)
            .service(update_article)
            .service(delete_article)
            .data(AppState {
                db: db_addr.clone(),
            })
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
