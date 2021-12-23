use diesel::{
    connection::Connection,
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};

pub fn run_migrations(database_url: &str) {
    embed_migrations!();
    let connection = PgConnection::establish(database_url).expect("Error connectiong to Database");
    embedded_migrations::run_with_output(&connection, &mut std::io::stdout())
        .expect("Error running migrations");
}

pub fn get_pool(database_url: &str) -> Pool<ConnectionManager<PgConnection>> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder()
        .build(manager)
        .expect("Error building a connection pool")
}
