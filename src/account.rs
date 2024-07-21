use std::time::Duration;
use reqwest::{Client, header, Error};

pub async fn get_http_post(key: &'static str, web: &str, that: bool) -> Result<String, Error> {
    let http = Client::new();
    if that {
        let head = "application/x-www-form-urlencoded;charset=utf-8";
        let res = http
            .post(web)
            .timeout(Duration::from_secs(100))
            .header(header::CONTENT_TYPE, head)
            .body(key)
            .send()
            .await?
            .text()
            .await?;
        Ok(res)
    }else{
        let head = "application/json;charset=utf-8";
        let res = http
            .post(web)
            .timeout(Duration::from_secs(100))
            .header(header::CONTENT_TYPE, head)
            .header(header::ACCEPT, head)
            .body(key)
            .send()
            .await?
            .text()
            .await?;
        Ok(res)
    }
}

pub async fn get_http_get(key: &'static str, web: &str) -> Result<String, Error> {
    let http = Client::new();
    let res = http
        .get(web)
        .timeout(Duration::from_secs(100))
        .header(header::AUTHORIZATION, format!("{} {}", "Bearer", key))
        .send()
        .await?
        .text()
        .await?;
    Ok(res)
}