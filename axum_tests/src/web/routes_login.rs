use crate:: {web, Error, Result};
use axum::{
    Json, 
    routing::post, 
    Router
};
use serde::Deserialize;
use serde_json::Value;
use serde_json::json;

pub fn routes() -> Router {
    Router::new().route("/api/login", post(api_login))
}

async fn api_login(payload: Json<LoginPayLoad>) -> Result<Json<Value>> {
    println!("->> {:<12} - api_login", "HANDLER");

    // TODO: Implement readl db/auth logic.
    if payload.username != "demo1" || payload.pwd != "123" {
        return Err(Error::LoginFail);
    }

    // TODO: Set cookies

    // Create the succes body.
    let body = Json(json!({
        "result": {
            "sucesss": true
        }
    }));

    Ok(body)
}

#[derive(Deserialize)]
struct LoginPayLoad {
    username: String,
    pwd:String,
}
