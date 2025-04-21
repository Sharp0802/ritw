mod lazy;
mod models;
mod routes;
mod services;

use crate::models::{Model, User};
use axum::Router;
use axum::routing::*;
use services::DB;
use tower::ServiceBuilder;
use tower_http::compression::CompressionLayer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("connecting to postgres...");

    DB::init().await?;

    println!("postgres connection established");

    User::up().await?;

    println!("database initialized");

    let app = Router::new()
        .route("/{file}", get(routes::default))
        .route("/signup", post(routes::signup_post))
        .route("/signin", post(routes::signin_post))
        .route("/signout", post(routes::signout_post))
        .layer(
            ServiceBuilder::new().layer(
                CompressionLayer::new()
                    .gzip(true)
                    .deflate(true)
                    .br(true)
                    .zstd(true),
            ),
        );
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();

    println!("listening on {}...", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();

    Ok(())
}
