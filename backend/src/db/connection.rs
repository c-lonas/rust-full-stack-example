use sqlx::{Pool, MySql};
use std::{env};


pub async fn pool() -> Pool<MySql> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = Pool::connect(&database_url).await.unwrap(); // Handle this unwrap 
    pool
}