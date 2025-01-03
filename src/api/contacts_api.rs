use serde_json::{json, Value};
use std::error::Error;

use super::{ApiClient, ContactOperationType, Wxid};

impl ApiClient {
    impl_params_api!(
    /// Fetch contacts list API
    ///
    /// Wrapper of calling `/contacts/fetchContactsList` API of the gewe service.
    /// Retrieves the contact list.
    ///
    /// Notice:
    /// 1. This is a time-consuming interface, try to use the cached version [`fetch_contacts_list_cache`] if possible.
    ///
    /// # Route
    ///
    /// /contacts/fetchContactsList
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
    ///     use rgewe::api::fetch_contacts_list;
    ///
    ///     let app_id = "your_app_id";  // Application identifier
    ///     let value = fetch_contacts_list(app_id).await.unwrap();
    ///     assert!(value.get("ret").unwrap().as_u64().unwrap() == 200);
    /// }
    /// ```
    fetch_contacts_list,
    "/contacts/fetchContactsList",
    ("appId", app_id, &str));
    // v0.1.0
    // pub async fn fetch_contacts_list(app_id: &str) -> Result<Value, Box<dyn Error>> {
    //     let params = json!({
    //         "appId": app_id,
    //     });
    //     util::gewe_post_json("/contacts/fetchContactsList", Some(params)).await
    // }

    impl_params_api!(
    /// Fetch cached contacts list API
    ///
    /// Wrapper of calling `/contacts/fetchContactsListCache` API of the gewe service.
    /// Retrieves the cached contact list (last for 10 mins).
    ///
    /// # Route
    ///
    /// /contacts/fetchContactsListCache
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
    ///     use rgewe::api::fetch_contacts_list_cache;
    ///
    ///     let app_id = "your_app_id";  // Application identifier
    ///     let value = fetch_contacts_list_cache(app_id).await.unwrap();
    ///     assert!(value.get("ret").unwrap().as_u64().unwrap() == 200);
    /// }
    /// ```
    fetch_contacts_list_cache,
    "/contacts/fetchContactsListCache",
    ("appId", app_id, &str));
    // v0.1.0
    // pub async fn fetch_contacts_list_cache(app_id: &str) -> Result<Value, Box<dyn Error>> {
    //     let params: Value = json!({
    //         "appId": app_id,
    //     });
    //     util::gewe_post_json("/contacts/fetchContactsListCache", Some(params)).await
    // }

    impl_params_api!(
    /// Search friend API
    ///
    /// Wrapper of calling `/contacts/search` API of the gewe service.
    /// Searches for a contact using the given keyword.
    ///
    /// # Route
    ///
    /// /contacts/search
    ///
    /// # Parameters
    ///
    /// - `app_id` - The application identifier associated with the user.
    /// - `keyword` - The search keyword to locate a contact (e.g. phone number or wechat id).
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// #[tokio::main]
    /// async fn main() {
    ///     use rgewe::api::search_friend;
    ///
    ///     let app_id = "your_app_id"; // Application identifier
    ///     let keyword = "1234567";    // Phone contact, phone number, wechat id alias, etc.
    ///     let value = search_friend(app_id, keyword).await.unwrap();
    ///     assert!(value.get("ret").unwrap().as_u64().unwrap() == 200);
    /// }
    /// ```
    search_friend,
    "/contacts/search",
    ("appId", app_id, &str),
    ("contactsInfo", keyword, &str));
    // v0.1.0
    // pub async fn search_friend(app_id: &str, keyword: &str) -> Result<Value, Box<dyn Error>> {
    //     let params = json!({
    //         "appId": app_id,
    //         "contactsInfo": keyword,
    //     });
    //     util::gewe_post_json("/contacts/search", Some(params)).await
    // }

    impl_params_api!(
    /// Add friend API
    ///
    /// Wrapper of calling `/contacts/search` API of the gewe service.
    /// TODO
    ///
    /// # Route
    ///
    /// /contacts/search
    ///
    /// # Parameters
    ///
    /// TODO
    /// - `app_id` - The application identifier associated with the user.
    ///
    /// # Examples
    ///
    /// TODO
    add_friend,
    "/contacts/search",
    ("appId", app_id, &str),
    ("scene", scene, i32),
    ("option", option, i32),
    ("v3", v3, &str),
    ("v4", v4, &str),
    ("content", content, &str));
    // v0.1.0
    // pub async fn add_friend(
    //     app_id: &str,
    //     scene: i32,
    //     option: i32,
    //     v3: &str,
    //     v4: &str,
    //     content: &str,
    // ) -> Result<Value, Box<dyn Error>> {
    //     let params = json!({
    //         "appId": app_id,
    //         "scene": scene,
    //         "option": option,
    //         "v3": v3,
    //         "v4": v4,
    //         "content": content,
    //     });
    //     util::gewe_post_json("/contacts/search", Some(params)).await
    // }

    impl_params_api!(
    /// Delete friend API
    ///
    /// Wrapper of calling `/contacts/deleteFriend` API of the gewe service.
    /// Deletes a contact by wxid.
    ///
    /// # Route
    ///
    /// /contacts/deleteFriend
    ///
    /// # Parameters
    ///
    /// - `app_id` - The application identifier associated with the user.
    /// - `wxid` - The unique wxid
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// #[tokio::main]
    /// async fn main() {
    ///     use rgewe::api::delete_friend;
    ///     use rgewe::user::Wxid;
    ///
    ///     let app_id = "your_app_id";  // Application identifier
    ///     let wxid = Wxid::try_from("wxid_example").unwrap();  // WeChat ID of the contact
    ///     let value = delete_friend(app_id, &wxid).await.unwrap();
    ///     assert!(value.get("ret").unwrap().as_u64().unwrap() == 200);
    /// }
    /// ```
    delete_friend,
    "/contacts/deleteFriend",
    ("appId", app_id, &str),
    ("wxid", wxid, &Wxid));
    // v0.1.0
    // pub async fn delete_friend(app_id: &str, wxid: &Wxid) -> Result<Value, Box<dyn Error>> {
    //     let params = json!({
    //         "appId": app_id,
    //         "wxid": wxid,
    //     });
    //     util::gewe_post_json("/contacts/deleteFriend", Some(params)).await
    // }

    impl_params_api!(
    /// Upload phone contacts API
    ///
    /// Wrapper of calling `/contacts/uploadPhoneAddressList` API of the gewe service.
    /// Used to add/remove a list of phone numbers representing contacts.
    ///
    /// # Route
    ///
    /// /contacts/uploadPhoneAddressList
    ///
    /// # Parameters
    ///
    /// - `app_id` - The application identifier associated with the user.
    /// - `phones` - A vector of phone numbers
    /// - `op` - The operation type, either [`ContactOperationType::Add`] or [`ContactOperationType::Remove`].
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// #[tokio::main]
    /// async fn main() {
    ///     use rgewe::api::{upload_phone_contacts, ContactOperationType};
    ///
    ///     let app_id = "your_app_id";  // Application identifier
    ///     let phones = vec!["1234567".to_string(), "7654321".to_string()];
    ///     let op = ContactOperationType::Add; // Add the contacts
    ///
    ///     let value = upload_phone_contacts(app_id, phones, op).await.unwrap();
    ///     assert!(value.get("ret").unwrap().as_u64().unwrap() == 200);
    /// }
    /// ```
    upload_phone_contacts,
    "/contacts/uploadPhoneAddressList",
    ("appId", app_id, &str),
    ("phones", phones, Vec<String>),
    ("opType", op, ContactOperationType));
    // v0.1.0
    // pub async fn upload_phone_contacts(
    //     app_id: &str,
    //     phones: Vec<String>,
    //     op: ContactOperationType,
    // ) -> Result<Value, Box<dyn Error>> {
    //     let params = json!({
    //         "appId": app_id,
    //         "phones": phones,
    //         "opType": op as u32,
    //     });
    //     util::gewe_post_json("/contacts/uploadPhoneAddressList", Some(params)).await
    // }

    impl_params_api!(
    /// Set friend chat-only permissions API
    ///
    /// Wrapper of calling `/contacts/setFriendPermissions` API of the gewe service.
    /// Enables or disables chat-only permissions for a contact.
    ///
    /// # Route
    ///
    /// /contacts/setFriendPermissions
    ///
    /// # Parameters
    ///
    /// - `app_id` - The application identifier associated with the user.
    /// - `wxid` - The unique WeChat ID of the contact.
    /// - `only_chat` - Whether to enable or disable chat-only (true or false).
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// #[tokio::main]
    /// async fn main() {
    ///     use rgewe::api::set_friend_only_chat;
    ///     use rgewe::user::Wxid;
    ///
    ///     let app_id = "your_app_id";  // Application identifier
    ///     let wxid = Wxid::try_from("wxid_example").unwrap();  // WeChat ID of the contact
    ///     let only_chat = true;  // Enable friend-only chat
    ///     let value = set_friend_only_chat(app_id, &wxid, only_chat).await.unwrap();
    ///     assert!(value.get("ret").unwrap().as_u64().unwrap() == 200);
    /// }
    /// ```
    set_friend_only_chat,
    "/contacts/setFriendPermissions",
    ("appId", app_id, &str),
    ("wxid", wxid, &Wxid),
    ("onlyChat", only_chat, bool));
    // v.0.1.0
    // pub async fn set_friend_only_chat(
    //     app_id: &str,
    //     wxid: &Wxid,
    //     only_chat: bool,
    // ) -> Result<Value, Box<dyn Error>> {
    //     // POST /contacts/setFriendPermissions
    //     let params = json!({
    //         "appId": app_id,
    //         "wxid": wxid,
    //         "onlyChat": only_chat,
    //     });
    //     util::gewe_post_json("/contacts/setFriendPermissions", Some(params)).await
    // }

    impl_params_api!(
    /// Set friend remark API
    ///
    /// Wrapper of calling `/contacts/setFriendRemark` API of the gewe service.
    /// Updates the remark (alias) for a specific contact.
    ///
    /// # Route
    ///
    /// /contacts/setFriendRemark
    ///
    /// # Parameters
    ///
    /// - `app_id` - The application identifier associated with the user.
    /// - `wxid` - The unique WeChat ID of the contact.
    /// - `remark` - The new remark to set for the contact.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// #[tokio::main]
    /// async fn main() {
    ///     use rgewe::api::set_friend_remark;
    ///     use rgewe::user::Wxid;
    ///
    ///     let app_id = "your_app_id";  // Application identifier
    ///     let wxid = Wxid::try_from("wxid_example").unwrap();  // WeChat ID of the contact
    ///     let remark = "Ana";  // New remark for the contact
    ///
    ///     let value = set_friend_remark(app_id, &wxid, remark).await.unwrap();
    ///     assert!(value.get("ret").unwrap().as_u64().unwrap() == 200);
    /// }
    /// ```
    set_friend_remark,
    "/contacts/setFriendRemark",
    ("appId", app_id, &str),
    ("wxid", wxid, &Wxid),
    ("remark", remark, &str));
    // v0.1.0
    // pub async fn set_friend_remark(
    //     app_id: &str,
    //     wxid: &Wxid,
    //     remark: &str,
    // ) -> Result<Value, Box<dyn Error>> {
    //     // POST /contacts/setFriendRemark
    //     let params = json!({
    //         "appId": app_id,
    //         "wxid": wxid,
    //         "remark": remark,
    //     });
    //     util::gewe_post_json("/contacts/setFriendRemark", Some(params)).await
    // }

    impl_params_api!(
    /// Get brief information for a single contact API
    ///
    /// Wrapper of calling `/contacts/getBriefInfo` API of the gewe service.
    /// Retrieves brief information for a specific contact by Wxid.
    ///
    /// # Route
    ///
    /// /contacts/getBriefInfo
    ///
    /// # Parameters
    ///
    /// - `app_id` - The application identifier associated with the user.
    /// - `wxid` - wxid of the contact.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// #[tokio::main]
    /// async fn main() {
    ///     use rgewe::api::get_brief_single;
    ///     use rgewe::user::Wxid;
    ///
    ///     let app_id = "your_app_id";  // Application identifier
    ///     let wxid = Wxid::try_from("wxid_example").unwrap();  // WeChat ID of the contact
    ///     let value = get_brief_single(app_id, &wxid).await.unwrap();
    ///     assert!(value.get("ret").unwrap().as_u64().unwrap() == 200);
    /// }
    /// ```
    get_brief_single,
    "/contacts/getBriefInfo",
    ("appId", app_id, &str),
    ("wxids", wxid, &Wxid));
    // v0.1.0
    // pub async fn get_brief_single(app_id: &str, wxid: &Wxid) -> Result<Value, Box<dyn Error>> {
    //     // POST /contacts/getBriefInfo
    //     let params = json!({
    //         "appId": app_id,
    //         "wxids": vec![wxid],
    //     });
    //     util::gewe_post_json("/contacts/getBriefInfo", Some(params)).await
    // }

    impl_params_api!(
    /// Get brief information for multiple contacts API
    ///
    /// Wrapper of calling `/contacts/getBriefInfo` API of the gewe service.
    /// Retrieves brief information for multiple contacts
    ///
    /// # Route
    ///
    /// /contacts/getBriefInfo
    ///
    /// # Parameters
    ///
    /// - `app_id` - The application identifier associated with the user.
    /// - `wxids` - A vector of wxid
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// #[tokio::main]
    /// async fn main() {
    ///     use rgewe::api::get_brief_list;
    ///     use rgewe::user::Wxid;
    ///
    ///     let app_id = "your_app_id";  // Application identifier
    ///     let wxids = vec![
    ///         Wxid::try_from("wxid_example1").unwrap(),
    ///         Wxid::try_from("wxid_example2").unwrap()
    ///     ];
    ///     let value = get_brief_list(app_id, wxids).await.unwrap();
    ///     assert!(value.get("ret").unwrap().as_u64().unwrap() == 200);
    /// }
    /// ```
    get_brief_list,
    "/contacts/getBriefInfo",
    ("appId", app_id, &str),
    ("wxids", wxids, Vec<Wxid>));
    // v0.1.0
    // pub async fn get_brief_list(app_id: &str, wxids: Vec<Wxid>) -> Result<Value, Box<dyn Error>> {
    //     let params = json!({
    //         "appId": app_id,
    //         "wxids": wxids,
    //     });
    //     util::gewe_post_json("/contacts/getBriefInfo", Some(params)).await
    // }
}
