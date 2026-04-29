#![allow(unused)]

use anyhow::Result;
use serde_json::json;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;
    hc.do_get("/hello?name=Mike").await?.print().await?;

    let req_login = hc.do_post(
        "/api/login",
        json!({
            "username": "demo1",
            "pwd": "123"
        })
    ); 
    req_login.await?.print().await?;

    hc.do_get("/hello?name=Mike").await?.print().await?;

    Ok(())
}
