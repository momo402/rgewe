use serde_json::{json, Value};
use std::error::Error;

use super::ApiClient;

impl ApiClient {
    impl_params_api!(
    /// Sync favor API
    /// TODO: need add doc
    sync_favor,
    "/favor/sync",
    ("appId", app_id, &str),
    ("syncKey", sync_key, &str));
    // v0.1.0
    // pub async fn sync_favor(app_id: &str, sync_key: &str) -> Result<Value, Box<dyn Error>> {
    //     let params = json!({
    //         "appId": app_id,
    //         "syncKey": sync_key,
    //     });
    //     util::gewe_post_json("/favor/sync", Some(params)).await
    // }

    impl_params_api!(
    /// Get favor content API
    /// TODO: need add doc
    get_favor_content,
    "/favor/getContent",
    ("appId", app_id, &str),
    ("favId", fav_id, i32));
    // v0.1.0
    // pub async fn get_favor_content(app_id: &str, fav_id: i32) -> Result<Value, Box<dyn Error>> {
    //     let params = json!({
    //         "appId": app_id,
    //         "favId": fav_id,
    //     });
    //     util::gewe_post_json("/favor/getContent", Some(params)).await
    // }

    impl_params_api!(
    /// Delete favor with favor id API
    /// TODO: need add doc
    delete_favor,
    "/favor/delete",
    ("appId", app_id, &str),
    ("favId", fav_id, i32));
    // v0.1.0
    // pub async fn delete_favor(app_id: &str, fav_id: i32) -> Result<Value, Box<dyn Error>> {
    //     let params = json!({
    //         "appId": app_id,
    //         "favId": fav_id,
    //     });
    //     util::gewe_post_json("/favor/delete", Some(params)).await
    // }
}
