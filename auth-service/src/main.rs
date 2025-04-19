mod db;
mod models;
mod services;
mod utils;
// use axum::{routing::{get, post}, Router};
use axum::{
    Router,
    routing::{get, post},
};
use db::Db;
use dotenvy::dotenv;
use services::auth::{ login, register};
use std::env;
// use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();
    // let db = Db::new().await;
    let db = match Db::new().await {
        Ok(db) => db,
        Err(e) => {
            eprintln!("Database initialization failed: {}", e);
            std::process::exit(1); // Optional: exit app if DB is critical
        }
    };

    let port = env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse::<u16>()
        .expect("PORT must be a number");

    let app = Router::new()
        .route("/health", post(|| async { "OK" }))
        .route("/", get(|| async { "Hello, World!" }))
        .route("/login", post(login))
        .route("/register", post(register))
        // .route("/user", get(get_user_handler))
        // .route("/signup", post(signup_handler)) // To implement similarly
        .with_state(db.clone());

    // let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    // println!("Listening on {}", addr);
    // axum::serve::bind(&addr).serve(app.into_make_service()).await.unwrap();
    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(("0.0.0.0", port))
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
