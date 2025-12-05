mod routing;

use routing::routes;
use tokio::net::TcpListener;
use sqlx::mysql::MySqlPoolOptions;
use dotenvy::dotenv;
use std::env;
use axum::Router;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");

    let db_pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to database");


    let app: Router = routes::create_routes(db_pool.clone());

    let listener = TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();


    println!("Using DB: {}", database_url);
    println!("API läuft auf http://localhost:8080");


    axum::serve(listener, app)
        .await
        .unwrap();
}
