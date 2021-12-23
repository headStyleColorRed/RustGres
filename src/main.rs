#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

mod actors;
mod db_utils;
mod models;
mod schema;

use actix_web::{
    delete, get, post, put,
    web::{self, Data, Json, Path},
    App, HttpResponse, HttpServer, Responder,
};

use actix::SyncArbiter;
use actors::db::{Create, DBActor, Delete, GetArticles, GetAllArticles, Publish, Update};
use db_utils::{get_pool, run_migrations};
use dotenv::dotenv;
use models::{AppState, ArticleData};
use std::env;
use uuid::Uuid;

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
            .route("/", web::get().to(health))
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

async fn health() -> impl Responder {
    HttpResponse::Ok().body("Server is up and running")
}

#[post("/new")]
async fn create_article(article: Json<ArticleData>, state: Data<AppState>) -> impl Responder {
    let db = state.as_ref().db.clone();
    let article: ArticleData = article.into_inner();
    let create = Create {
        title: article.title,
        body: article.body,
    };

    match db.send(create).await {
        Ok(Ok(article)) => HttpResponse::Ok().json(article),
        _ => HttpResponse::InternalServerError().json("Something went wrong"),
    }
}

#[post("/{uuid}/publish")]
async fn publish_article(Path(uuid): Path<Uuid>, state: Data<AppState>) -> impl Responder {
    let db = state.as_ref().db.clone();
    let publish = Publish { uuid };

    match db.send(publish).await {
        Ok(Ok(article)) => HttpResponse::Ok().json(article),
        Ok(Err(_)) => HttpResponse::NotFound().json("Article not found"),
        _ => HttpResponse::InternalServerError().json("Something went wrong"),
    }
}

#[delete("/{uuid}")]
async fn delete_article(Path(uuid): Path<Uuid>, state: Data<AppState>) -> impl Responder {
    let db = state.as_ref().db.clone();
    let delete = Delete { uuid };

    match db.send(delete).await {
        Ok(Ok(article)) => HttpResponse::Ok().json(article),
        Ok(Err(_)) => HttpResponse::NotFound().json("Article not found"),
        _ => HttpResponse::InternalServerError().json("Something went wrong"),
    }
}

#[put("/{uuid}")]
async fn update_article(
    Path(uuid): Path<Uuid>,
    article: Json<ArticleData>,
    state: Data<AppState>,
) -> impl Responder {
    let db = state.as_ref().db.clone();
    let update_article = article.into_inner();
    let update = Update {
        uuid,
        title: update_article.title,
        body: update_article.body,
    };

    match db.send(update).await {
        Ok(Ok(article)) => HttpResponse::Ok().json(article),
        Ok(Err(_)) => HttpResponse::NotFound().json("Article not found"),
        _ => HttpResponse::InternalServerError().json("Something went wrong"),
    }
}

#[get("/published")]
async fn get_published(state: Data<AppState>) -> impl Responder {
    let db = state.as_ref().db.clone();

    match db.send(GetArticles).await {
        Ok(Ok(articles)) => HttpResponse::Ok().json(articles),
        _ => HttpResponse::InternalServerError().json("Something went wrong"),
    }
}

#[get("/all")]
async fn get_all(state: Data<AppState>) -> impl Responder {
    let db = state.as_ref().db.clone();

    match db.send(GetAllArticles).await {
        Ok(Ok(articles)) => HttpResponse::Ok().json(articles),
        _ => HttpResponse::InternalServerError().json("Something went wrong"),
    }
}