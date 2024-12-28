use crate::{user::Wxid, util};
use serde_json::{json, Value};
use std::error::Error;

/// Add label/tag API
///
/// Wrapper of calling `/label/add` API of the service.
/// Add new label_name to tags
///
/// # Route
///
/// `/label/add`
///
/// # Parameters
///
/// - `app_id` - The application identifier associated with the user.
/// - `label_name` - The name of the label to be added.
///
/// # Examples
///
/// ```rust,no_run
/// #[tokio::main]
/// async fn main() {
///     use rgewe_api::api;
///     let app_id = "your_app_id"; // Application identifier
///     let label_name = "test_add3"; // Label to add
///     let value = api::add_label(app_id, label_name).await.unwrap();
///     assert!(value.get("ret").unwrap().as_u64().unwrap() == 200);
/// }
/// ```
pub async fn add_label(app_id: &str, label_name: &str) -> Result<Value, Box<dyn Error>> {
    let params = json!({
        "appId": app_id,
        "labelName": label_name,
    });
    util::gewe_post_json("/label/add", Some(params)).await
}

/// Delete label/tag API
///
/// Wrapper of calling `/label/delete` API of the service.
/// Delete label with id
///
/// # Route
///
/// `/label/add`
///
/// # Parameters
///
/// - `app_id` - The application identifier associated with the user.
/// - `label_id` - The delete label id
///
/// # Examples
///
/// ```rust,no_run
/// #[tokio::main]
/// async fn main() {
///     use rgewe_api::api;
///     let app_id = "your_app_id"; // Application identifier
///     let label_id = "6"; // Label' id to delete
///     let value = api::delete_label(app_id, label_id).await.unwrap();
///     assert!(value.get("ret").unwrap().as_u64().unwrap() == 200);
/// }
/// ```
pub async fn delete_label(app_id: &str, label_ids: &str) -> Result<Value, Box<dyn Error>> {
    let params = json!({
        "appId": app_id,
        "labelIds": label_ids,
    });
    util::gewe_post_json("/label/delete", Some(params)).await
}

/// List label/tag API
///
/// Wrapper of calling `/label/add` API of the service.
/// Fetch and list labels
///
/// # Route
///
/// `/label/list`
///
/// # Parameters
///
/// - `app_id` - The application identifier associated with the user.
///
/// # Examples
///
/// ```rust,no_run
/// #[tokio::main]
/// async fn main() {
///     use rgewe_api::api;
///     let app_id = "your_app_id"; // Application identifier
///     let value = api::list_labels(app_id).await.unwrap();
///     assert!(value.get("ret").unwrap().as_u64().unwrap() == 200);
/// }
/// ```
pub async fn list_labels(app_id: &str) -> Result<Value, Box<dyn Error>> {
    let params = json!({
        "appId": app_id,
    });
    util::gewe_post_json("/label/list", Some(params)).await
}

/// Modify members of label/tag API
///
/// Wrapper of calling `/label/add` API of the service.
/// Modify label members
///
/// Notice:
/// 1. Reset labels of given wxids (not add/delete)
///
/// # Route
///
/// `/label/modifyMemberList`
///
/// # Parameters
///
/// - `app_id` - The application identifier associated with the user.
/// - `label_ids` - The labels vector to reset
/// - "wxids"
///
/// # Examples
///
/// ```rust,no_run
/// #[tokio::main]
/// async fn main() {
///     use rgewe_api::api;
///     use rgewe_api::user::Wxid;
///
///     let app_id = "your_app_id"; // Application identifier
///     let label_ids = vec!["6".to_string(),"7".to_string()];
///     let wxid = vec![Wxid::try_from("test_wxid").unwrap()];
///     let value = api::modify_label_members(app_id, label_ids, wxid).await.unwrap();
///     assert!(value.get("ret").unwrap().as_u64().unwrap() == 200);
/// }
/// ```
pub async fn modify_label_members(
    app_id: &str,
    label_ids: Vec<String>,
    wxids: Vec<Wxid>,
) -> Result<Value, Box<dyn Error>> {
    let flatten_label_ids = label_ids
        .iter()
        .map(|wxid| wxid.to_string())
        .collect::<Vec<String>>()
        .join(",");
    let params = json!({
        "appId": app_id,
        "labelIds": flatten_label_ids,
        "wxIds": wxids,
    });
    util::gewe_post_json("/label/modifyMemberList", Some(params)).await
}
