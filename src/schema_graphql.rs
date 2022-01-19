use std::sync::Arc;
use juniper::{RootNode, graphql_object, FieldResult};
use juniper::{EmptyMutation, EmptySubscription};
use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};

pub struct Context {
    pub db_pool: Arc<Pool<ConnectionManager<PgConnection>>>,
}
pub struct QueryRoot;
#[graphql_object(Context = Context)]
impl QueryRoot {
    fn hello(_context: &Context) -> FieldResult<String> {
        Ok(format!("Hello World!"))
    }
}

pub type SchemaGraphQL = RootNode<'static, QueryRoot, EmptyMutation<Context>, EmptySubscription<Context>>;

pub fn create_schema() -> SchemaGraphQL {
    SchemaGraphQL::new(QueryRoot, EmptyMutation::new(), EmptySubscription::new())
}