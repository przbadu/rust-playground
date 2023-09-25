#![allow(unused)]

use anyhow::Result;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8000")?;

    hc.do_get("/hello/name=John!").await?.print().await?;
    hc.do_get("/hello2/doe").await?.print().await?;

    Ok(())
}
