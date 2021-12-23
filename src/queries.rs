
use actix_web::{
    delete, get, post, put,
    web::{Data, Json, Path},
    HttpResponse, Responder,
};
use crate::actors::db::{Create, Delete, GetArticles, GetAllArticles, Publish, Update};
use crate::models::{AppState, ArticleData};
use uuid::Uuid;


#[get("/")]
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