use super::Error;
use sqlx::PgPool;
use std::env;

pub async fn connect_to_db() -> Result<sqlx::PgPool, Error> {
    let pool = PgPool::connect(&env::var("DATABASE_URL")?).await?;

    sqlx::migrate!().run(&pool).await?;

    Ok(pool)
}

pub async fn connect_to_dev_db() -> Result<sqlx::PgPool, Error> {
    match dotenvy::dotenv() {
        Ok(_) => println!("Dev vars successfully loaded"),
        Err(_) => println!("Failed to load dev vars"),
    };

    let pool = PgPool::connect(&env::var("DEV_DATABASE_URL")?).await?;

    Ok(pool)
}
