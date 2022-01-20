use std::sync::Arc;

use actix_web::http::Method;
use actix_web::{web, Error, HttpRequest, HttpResponse};
use juniper::http::{playground::playground_source, GraphQLRequest};

use crate::schema_graphql::{DbPool, Context};
use crate::schema_graphql::SchemaGraphQL;

pub async fn playground() -> HttpResponse {
    let html = playground_source("/graphql", None);
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

pub async fn graphql(
    req: HttpRequest,
    st: web::Data<Arc<SchemaGraphQL>>,
    data_query: Option<web::Query<GraphQLRequest>>,
    data_body: Option<web::Json<GraphQLRequest>>,
    db_pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {

    // fetch data from
    // query string if this is a GET
    // body if this is a POST
    let data = match *req.method() {
        Method::GET => data_query.unwrap().into_inner(),
        _ => data_body.unwrap().into_inner(),
    };

    let db_pool = (*db_pool).clone();
    let ctx = Context { db_pool };
    let res = data.execute(&st, &ctx).await;

    Ok(HttpResponse::Ok().json(res))
}
