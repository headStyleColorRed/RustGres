use crate::models::{Article, NewArticle};
use crate::schema::articles::dsl::{articles, body, published, title, uuid as article_id};
use actix::{Actor, Handler, Message, SyncContext};
use diesel::prelude::*;
use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};
use uuid::Uuid;

// Database Actor
pub struct DBActor(pub Pool<ConnectionManager<PgConnection>>);

impl Actor for DBActor {
    type Context = SyncContext<Self>;
}

// Create an article
#[derive(Message)]
#[rtype(result = "QueryResult<Article>")]
pub struct Create {
    pub title: String,
    pub body: String,
}
impl Handler<Create> for DBActor {
    type Result = QueryResult<Article>;

    fn handle(&mut self, msg: Create, _: &mut Self::Context) -> Self::Result {
        let connection = self.0.get().expect("Unable to get a connection");
        let new_article = NewArticle {
            uuid: Uuid::new_v4(),
            title: msg.title,
            body: msg.body,
        };

        diesel::insert_into(articles)
            .values(new_article)
            .get_result::<Article>(&connection)
    }
}

// Update an article
#[derive(Message)]
#[rtype(result = "QueryResult<Article>")]
pub struct Update {
    pub uuid: Uuid,
    pub title: String,
    pub body: String,
}
impl Handler<Update> for DBActor {
    type Result = QueryResult<Article>;

    fn handle(&mut self, msg: Update, _: &mut Self::Context) -> Self::Result {
        let connection = self.0.get().expect("Unable to get a connection");

        diesel::update(articles)
            .filter(article_id.eq(msg.uuid))
            .set((title.eq(msg.title), body.eq(msg.body)))
            .get_result::<Article>(&connection)
    }
}

// Delete an article
#[derive(Message)]
#[rtype(result = "QueryResult<Article>")]
pub struct Delete {
    pub uuid: Uuid,
}
impl Handler<Delete> for DBActor {
    type Result = QueryResult<Article>;

    fn handle(&mut self, msg: Delete, _: &mut Self::Context) -> Self::Result {
        let connection = self.0.get().expect("Unable to get a connection");

        diesel::delete(articles)
            .filter(article_id.eq(msg.uuid))
            .get_result::<Article>(&connection)
    }
}

// Publish an article
#[derive(Message)]
#[rtype(result = "QueryResult<Article>")]
pub struct Publish {
    pub uuid: Uuid,
}
impl Handler<Publish> for DBActor {
    type Result = QueryResult<Article>;

    fn handle(&mut self, msg: Publish, _: &mut Self::Context) -> Self::Result {
        let connection = self.0.get().expect("Unable to get a connection");

        diesel::update(articles)
            .filter(article_id.eq(msg.uuid))
            .set(published.eq(true))
            .get_result::<Article>(&connection)
    }
}

// Retrieve published articles
#[derive(Message)]
#[rtype(result = "QueryResult<Vec<Article>>")]
pub struct GetArticles;
impl Handler<GetArticles> for DBActor {
    type Result = QueryResult<Vec<Article>>;

    fn handle(&mut self, _: GetArticles, _: &mut Self::Context) -> Self::Result {
        let conn = self.0.get().expect("Unable to get a connectio");
        articles
            .filter(published.eq(true))
            .get_results::<Article>(&conn)
    }
}
// Retrieve published articles
#[derive(Message)]
#[rtype(result = "QueryResult<Vec<Article>>")]
pub struct GetAllArticles;
impl Handler<GetAllArticles> for DBActor {
    type Result = QueryResult<Vec<Article>>;

    fn handle(&mut self, _: GetAllArticles, _: &mut Self::Context) -> Self::Result {
        let conn = self.0.get().expect("Unable to get a connectio");
        articles.get_results::<Article>(&conn)
    }
}
