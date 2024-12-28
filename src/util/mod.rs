use reqwest::{header::HeaderMap, Client};
use serde_json::Value;
use std::{collections::HashMap, error::Error};

const BASE_URL: &str = "http://localhost:2531/v2/api";
const HEADER_GEWE: &str = "X-GEWE-TOKEN";
const TOKEN: &str = "";

pub async fn bare_get() -> Result<HashMap<String, String>, Box<dyn Error>> {
    let resp = reqwest::get("https://httpbin.org/ip")
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    Ok(resp)
}

pub async fn gewe_post_json(route: &str, body: Option<Value>) -> Result<Value, Box<dyn Error>> {
    // Collect header from configuration
    // TODO configure token
    // TODO configure check
    let mut headers = HeaderMap::new();
    headers.insert(HEADER_GEWE, TOKEN.parse().unwrap());

    // Check json body
    if body.is_none() {
        // Empty json body
        // only allow ask for token
        if route != "/tools/getTokenId" {
            return Err(format!("Empty json body for route: {}", route).into());
        }
    }
    let url = format!("{}{}", BASE_URL, route);
    post_json(&url, headers, &body).await
}

pub fn build_no_proxy_client() -> Client {
    Client::builder().no_proxy().build().unwrap()
}

pub async fn post_json(
    url: &str,
    headers: HeaderMap,
    body: &Option<Value>,
) -> Result<Value, Box<dyn Error>> {
    // Build basic reqwest client
    // println!("{:#?}",body.clone().unwrap().to_string());

    // TODO: add configure for proxy
    // Use no proxy clent for test
    // let client = build_with_proxy_client();
    let client = build_no_proxy_client();

    let rest = client
        .post(url)
        // TODO: change headers to ref?
        .headers(headers)
        .json(body)
        .send()
        .await?
        .json()
        .await?;
    Ok(rest)
}
