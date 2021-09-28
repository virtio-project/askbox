use sqlx::{pool::Pool, postgres::Postgres};

pub type DbPool = Pool<Postgres>;
