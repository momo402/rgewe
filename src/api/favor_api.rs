use crate::util;
use serde_json::{json, Value};
use std::error::Error;

/// TODO: need test and add doc
pub async fn sync_favor(app_id: &str, sync_key: &str) -> Result<Value, Box<dyn Error>> {
    let params = json!({
        "appId": app_id,
        "syncKey": sync_key,
    });
    util::gewe_post_json("/favor/sync", Some(params)).await
}

/// TODO: need test and add doc
pub async fn get_favor_content(app_id: &str, fav_id: i32) -> Result<Value, Box<dyn Error>> {
    let params = json!({
        "appId": app_id,
        "favId": fav_id,
    });
    util::gewe_post_json("/favor/getContent", Some(params)).await
}

/// TODO: need test and add doc
pub async fn delete_favor(app_id: &str, fav_id: i32) -> Result<Value, Box<dyn Error>> {
    let params = json!({
        "appId": app_id,
        "favId": fav_id,
    });
    util::gewe_post_json("/favor/delete", Some(params)).await
}
