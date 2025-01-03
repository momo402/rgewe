use rgewe_api::api;
#[tokio::test]
async fn test_get_ip() {
    let fu = api::test_get_ip();
    assert!(fu.await.is_ok());
}

#[tokio::test]
async fn test_get_token() {
    let c = api::ApiClientBuilder::new().build();
    let ret = c.get_token().await.unwrap();
    println!("{:#?}", ret);
    let token = ret.get("data").unwrap().to_string();
    assert!(!token.is_empty());
}

#[tokio::test]
async fn test_get_profile() {
    let no_token_c = api::ApiClientBuilder::new().build();
    let token = no_token_c
        .get_token()
        .await
        .unwrap()
        .get("data")
        .unwrap()
        .to_string();

    let with_token_c = api::ApiClientBuilder::new().with_token(&token).build();
    let ret = with_token_c.get_profile("need_app_id").await.unwrap();
    println!("{:#?}", ret);
    let data = ret.get("data").unwrap().to_string();
    println!("{:#?}", data);
}
