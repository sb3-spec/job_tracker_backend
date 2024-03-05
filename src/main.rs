use std::{env, sync::Arc};

use models::db::connect_to_db;

const DEFAULT_WEB_PORT: u16 = 8080;

mod models;

#[tokio::main]
async fn main() {
    match dotenvy::dotenv() {
        Ok(_) => println!("Dev vars successfully loaded"),
        Err(_) => println!("Failed to load dev vars"),
    };

    let web_port: u16 = match env::var("PORT") {
        Ok(port) => port.parse::<u16>().unwrap(),
        Err(_) => DEFAULT_WEB_PORT,
    };

    let db = Arc::new(connect_to_db().await.expect("Cannot connect to db"));
}
