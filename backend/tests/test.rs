use anyhow::{Result, Ok};
use reqwest::{ Client, multipart::Form, multipart::Part };
use tokio::fs::File;
use tokio::io::AsyncReadExt; // for read_to_end()

/*
    Start testing with 'cargo watch -qcx 'shuttle run --external' --ignore backend/target --ignore backend/tests'
*/

#[tokio::test]
async fn test1() -> Result<(), anyhow::Error> {
    let client =  Client::builder()
        .pool_max_idle_per_host(0)
        .build()?;

    let mut file = File::open("./tests/test.jpg").await?;

    let mut contents = vec![];
    file.read_to_end(&mut contents).await?;

    let form = Form::new()
        .part("animation_type", Part::text("rotate-cw"))
        .part("image", Part::bytes(contents).file_name("test.jpg").mime_str("image/jpeg")?);


    let _res = client.post("http://0.0.0.0:8000/api")
        .multipart(form)
        .send()
        .await?
        .text()
        .await?;

    println!("Res: {}", _res);

    Ok(())
}

#[tokio::test]
async fn test2() -> Result<(), anyhow::Error> {
    let client =  Client::new();

    let _res = client.get("http://0.0.0.0:8000")
        .send()
        .await?
        .text()
        .await?;

    println!("Res: {}", _res);

    Ok(())
}