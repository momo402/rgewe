use rgewe_api::api;
#[tokio::test]
async fn test_bare_get() {
    assert!(api::bare_get().await.is_ok());
}

#[tokio::test]
async fn test_get_token() {
    let ret = api::get_token().await.unwrap();
    println!("{:#?}", ret);
    let token = ret.get("data").unwrap().to_string();
    assert!(!token.is_empty());
}

#[tokio::test]
async fn test_get_profile() {
    let ret = api::get_profile("").await.unwrap();
    println!("{:#?}", ret);
    ret.get("data").unwrap().to_string();
}
