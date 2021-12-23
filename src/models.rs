use crate::actors::db::DBActor;
use crate::schema::articles;
use actix::Addr;
use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub struct AppState {
    pub db: Addr<DBActor>,
}

/// Main article object
#[derive(Debug, Clone, Queryable, Serialize, Deserialize)]
pub struct Article {
    pub uuid: Uuid,
    pub title: String,
    pub body: String,
    pub published: bool,
}

/// Object to create a new article
#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[table_name = "articles"]
pub struct NewArticle {
    pub uuid: Uuid,
    pub title: String,
    pub body: String,
}

/// Object to capture the query body
#[derive(Serialize, Deserialize)]
pub struct ArticleData {
    pub title: String,
    pub body: String,
}