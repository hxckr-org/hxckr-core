use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};
use std::env;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

pub fn get_connection_pool() -> Pool<ConnectionManager<PgConnection>> {
    let url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(url);
    Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .expect("Could not build connection pool")
}
