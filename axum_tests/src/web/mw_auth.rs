use crate::ctx::Ctx;
use crate::web::AUTH_TOKEN;
use crate::{Error, Result};
use axum::body::Body;
use axum::http::Request;
use axum::middleware::Next;
use axum::response::Response;
use tower_cookies::Cookies;

pub async fn mw_require_auth(
    cookies: Cookies,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response> {
    println!("->> {:<12} - mw_require_auth", "MIDDLEWARE");

    let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());

    let (user_id, _exp, _sign) = auth_token
        .ok_or(Error::AuthFailNoAuthTokenCookie)
        .and_then(parse_token)?;

    req.extensions_mut().insert(Ctx::new(user_id));

    Ok(next.run(req).await)
}

fn parse_token(token: String) -> Result<(u64, String, String)> {
    let parts: Vec<&str> = token.splitn(3, '.').collect();
    if parts.len() != 3 {
        return Err(Error::AuthFailtokenwrongFormat);
    }

    // Mock token format: "user-{id}.exp.sign"
    let user_id = parts[0]
        .trim_start_matches("user-")
        .parse::<u64>()
        .map_err(|_| Error::AuthFailtokenwrongFormat)?;

    Ok((user_id, parts[1].to_string(), parts[2].to_string()))
}
