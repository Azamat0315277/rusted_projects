#![allow(unused)] // For beginning only

use axum::Router;
use axum::extract::Path;
use axum::extract::Query;
use axum::response::Html;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::routing::get_service;
use serde::Deserialize;
use tower_http::services::ServeDir;
use std::net::SocketAddr;
pub use self::error::{Error, Result};
mod error;
mod web;

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let routes_all = Router::new()
    .merge(routes_hello())
    .merge(web::routes_login::routes())
    .fallback_service(routes_static());

    // region: -- Start Server
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("->> LISTENING on {addr}\n");
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, routes_all).await?;
    // endregion: -- Start Server
    
    Ok(())
}

fn routes_hello() -> Router {
    Router::new()
    .route("/hello", get(handler_hello))
    .route("/hello2", get(handler_hello2))
}

fn routes_static() -> ServeDir {
    ServeDir::new("./")
}



#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello - {params:?}", "HANDLER");

    let name = params.name.as_deref().unwrap_or("World!");

    Html(format!("Hello <strong>{name}</strong>"))
}

async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello2 - {name:?}", "HANDLER");

    Html(format!("Hello <strong>{name}</strong>"))
}
