mod routing;
mod seed;

use routing::routes;
use tokio::net::TcpListener;
use sqlx::mysql::MySqlPoolOptions;
use dotenvy::dotenv;
use std::env;
use axum::Router;

#[tokio::main]
async fn main() {
    println!("Starting backend server...");

    // Lade .env Datei, falls vorhanden (lokale Entwicklung)
    dotenv().ok();

    // Environment Variablen aus Docker Compose verwenden
    let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());

    println!("on {}:{}", host, port);
    
    // Wir brauchen die DATABASE_URL, die wir in Docker Compose gesetzt haben
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");

    println!("Using database URL: {}", database_url);


    println!("Connecting to database...");
    // 1. Datenbankverbindung herstellen
    let db_pool = match MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            println!("✅ Database connection successful!");
            pool
        }
        Err(err) => {
            eprintln!("❌ Failed to connect to database: {}", err);
            // Wenn die Verbindung fehlschlägt, beenden wir das Programm mit einem Fehler-Code
            std::process::exit(1);
        }
    };

    println!("Running database migrations...");


    // Y
    println!("Dropping all tables...");

    let tables = sqlx::query_scalar::<_, String>(
        "
            SELECT table_name FROM information_schema.tables
            WHERE table_schema = DATABASE()
        "
    ).fetch_all(&db_pool).await.unwrap();

    sqlx::query("SET FOREIGN_KEY_CHECKS = 0;").execute(&db_pool).await.unwrap();

    for table in tables {
        let query = format!("DROP TABLE `{}`", table);
        println!("Dropping table: {}", query);
        sqlx::query(&query).execute(&db_pool).await.unwrap();
    }

    sqlx::query("SET FOREIGN_KEY_CHECKS = 1;").execute(&db_pool).await.unwrap();

    // Y



    //migrations::run_migrations(&db_pool).await.expect("Failed to run migration data");
    sqlx::migrate!("./migrations").run(&db_pool).await.expect("msg");

    println!("Running seed data...");
    seed::run_seeding(&db_pool).await.expect("Failed to run seed data");

    println!("Setting up routes...");
    
    let app: Router = routes::create_routes(db_pool.clone());

    // 2. TCP Listener binden
    // WICHTIG: Verwende 0.0.0.0, um außerhalb des Containers erreichbar zu sein
    let bind_addr = format!("{}:{}", host, port);

    println!("Binding to address: {}", bind_addr);

    let listener = match TcpListener::bind(&bind_addr).await {
        Ok(l) => {
            println!("✅ API listening on {}", bind_addr);
            l
        }
        Err(err) => {
            eprintln!("❌ Failed to bind listener to {}: {}", bind_addr, err);
            std::process::exit(1);
        }
    };
    
    println!("API läuft auf http://localhost:8090 (über Docker Port Forwarding)");

    // 3. Axum Server starten und blockieren
    axum::serve(listener, app)
        .await
        .expect("Server failed to run");
}