mod api;
mod models;
mod services;
mod utils;

use axum::{
    routing::{get, post, put, delete},
    Router, Extension,
};
use std::net::SocketAddr;
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .connect(&database_url)
        .await
        .expect("Failed to create pool");

    // Define your app with routes and handlers
    let app = Router::new()
        .route("/transactions", get(api::handlers::list_transactions).post(api::handlers::create_transaction))
        .route("/transactions/:id", get(api::handlers::get_transaction).put(api::handlers::update_transaction).delete(api::handlers::delete_transaction))
        .layer(Extension(pool)); // Pass the pool as a shared resource

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
