use crate:: {web, Error, Result};
use axum::{
    Json, 
    routing::post, 
    Router
};
use serde::Deserialize;
use serde_json::Value;
use serde_json::json;
use tower_cookies::Cookie;
use tower_cookies::Cookies;

pub fn routes() -> Router {
    Router::new().route("/api/login", post(api_login))
}

async fn api_login(cookies: Cookies, payload: Json<LoginPayLoad>) -> Result<Json<Value>> {
    println!("->> {:<12} - api_login", "HANDLER");

    // TODO: Implement readl db/auth logic.
    if payload.username != "demo1" || payload.pwd != "123" {
        return Err(Error::LoginFail);
    }

    cookies.add(Cookie::new(web::AUTH_TOKEN, "user-1.exp.sign"));

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
