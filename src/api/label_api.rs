use serde_json::{json, Value};
use std::error::Error;

use super::{ApiClient, Wxid};

impl ApiClient {
    impl_params_api!(
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
    ///     use rgewe::api;
    ///     let app_id = "your_app_id"; // Application identifier
    ///     let label_name = "test_add3"; // Label to add
    ///     let value = api::add_label(app_id, label_name).await.unwrap();
    ///     assert!(value.get("ret").unwrap().as_u64().unwrap() == 200);
    /// }
    /// ```
    add_label,
    "/label/add",
    ("appId", app_id, &str),
    ("labelName", label_name, &str));
    // v0.1.0
    // pub async fn add_label(app_id: &str, label_name: &str) -> Result<Value, Box<dyn Error>> {
    //     let params = json!({
    //         "appId": app_id,
    //         "labelName": label_name,
    //     });
    //     util::gewe_post_json("/label/add", Some(params)).await
    // }

    impl_params_api!(
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
    ///     use rgewe::api;
    ///     let app_id = "your_app_id"; // Application identifier
    ///     let label_id = "6"; // Label' id to delete
    ///     let value = api::delete_label(app_id, label_id).await.unwrap();
    ///     assert!(value.get("ret").unwrap().as_u64().unwrap() == 200);
    /// }
    /// ```
    delete_label,
    "/label/delete",
    ("appId", app_id, &str),
    ("labelIds", label_ids, &str));
    // v0.1.0
    // pub async fn delete_label(app_id: &str, label_ids: &str) -> Result<Value, Box<dyn Error>> {
    //     let params = json!({
    //         "appId": app_id,
    //         "labelIds": label_ids,
    //     });
    //     util::gewe_post_json("/label/delete", Some(params)).await
    // }

    impl_params_api!(
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
    ///     use rgewe::api;
    ///     let app_id = "your_app_id"; // Application identifier
    ///     let value = api::list_labels(app_id).await.unwrap();
    ///     assert!(value.get("ret").unwrap().as_u64().unwrap() == 200);
    /// }
    /// ```
    list_labels,
    "/label/list",
    ("appId", app_id, &str));
    // v0.1.0
    // pub async fn list_labels(app_id: &str) -> Result<Value, Box<dyn Error>> {
    //     let params = json!({
    //         "appId": app_id,
    //     });
    //     util::gewe_post_json("/label/list", Some(params)).await
    // }

    impl_params_api!(
    /// Modify members of label/tag API
    ///
    /// Wrapper of calling `/label/add` API of the service.
    /// Modify label members
    ///
    /// Notice:
    /// 1. Reset labels of given wxids (not add/delete)
    /// - **v0.2.0:**
    ///     - Uses the macro `impl_params_api!` for streamlined implementation.
    ///     - Accepts a pre-flattened labelids (e.g. "6,7").
    /// - **v0.1.0:**
    ///    - Accepts a vector of String.
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
    /// TODO: update label examples
    ///
    /// ```rust,no_run
    /// #[tokio::main]
    /// async fn main() {
    ///     use rgewe::api;
    ///     use rgewe::user::Wxid;
    ///
    ///     let app_id = "your_app_id"; // Application identifier
    ///     let label_ids = vec!["6".to_string(),"7".to_string()];
    ///     let wxid = vec![Wxid::try_from("test_wxid").unwrap()];
    ///     let value = api::modify_label_members(app_id, label_ids, wxid).await.unwrap();
    ///     assert!(value.get("ret").unwrap().as_u64().unwrap() == 200);
    /// }
    /// ```
    modify_label_members,
    "/label/modifyMemberList",
    ("appId", app_id, &str),
    ("labelIds", flatten_labelids, &str),
    ("wxIds", wxids, Vec<Wxid>));
    // v0.1.0
    // pub async fn modify_label_members(
    //     app_id: &str,
    //     label_ids: Vec<String>,
    //     wxids: Vec<Wxid>,
    // ) -> Result<Value, Box<dyn Error>> {
    //     let flatten_labelids = label_ids
    //         .iter()
    //         .map(|l| l.to_string())
    //         .collect::<Vec<String>>()
    //         .join(",");
    //     let params = json!({
    //         "appId": app_id,
    //         "labelIds": flatten_labelids,
    //         "wxIds": wxids,
    //     });
    //     util::gewe_post_json("/label/modifyMemberList", Some(params)).await
    // }
}
