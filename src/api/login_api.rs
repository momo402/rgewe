use crate::util;
use serde_json::{json, Value};
use std::error::Error;

/// Get token API
///
/// Wrapper of calling /tools/getTokenId API of gewe service.
/// The only one with no app_id parameter (application identifier).
///
/// # Route
///
/// /tools/getTokenId
///
/// # Parameters
///
/// # Examples
///
/// ```rust, no_run
/// #[tokio::main]
/// async fn main() {
///     use rgewe_api::api;
///     let value = api::get_token().await.unwrap();
///     assert!(value.get("ret").unwrap().as_u64().unwrap() == 200);
///     let token = value.get("data").unwrap().to_string();
///     println!("token: {}", token);
/// }
/// ```
pub async fn get_token() -> Result<Value, Box<dyn Error>> {
    util::gewe_post_json("/tools/getTokenId", None).await
}

/// Set callback URL API
///
/// Wrapper of calling /tools/setCallBack API of gewe service.
/// TODO: tigger some bug for Rust calling this API.
///
/// # Route
///
/// /tools/getTokenId
///
/// # Parameters
///
/// - `app_id` - The application identifier associated with the user.
///
/// # Examples
///
/// ```rust, no_run
/// #[tokio::main]
/// async fn main() {
///     use rgewe_api::api;
///     let token = "your_token"; // Authentication token e.g. From api::get_token
///     let callback_url = "your_callback_url"; // Callback URL to receive messages e.g. "http://127.0.0.1:18080/callback"
///     let value = api::set_call_back(token, callback_url).await.unwrap();
///     assert!(value.get("ret").unwrap().as_u64().unwrap() == 200);
/// }
/// ```
pub async fn set_call_back(token: &str, callback_url: &str) -> Result<Value, Box<dyn Error>> {
    let params = json!({
        "token": token,              // Authentication token
        "callbackUrl": callback_url, // Callback URL
    });
    util::gewe_post_json("/tools/setCallBack", Some(params)).await
}

/// Get login QR code API
///
/// Wrapper of calling /login/getLoginQrCode API of gewe service.
/// Used to get a login QR code for the user.
///
/// Notice:
///
/// 1. `app_id` should be emty string for the first time.
///     Gewe service detacts and creates a new `app_id`.
/// 2. Otherwise `app_id` should be **the same as** the last time logging in
/// 3. The QR code is valid for 230+ secs (ecah status).
/// 4. After calling this API, the user should scan the QR code
///     and call check_login_qr.
///
/// # Route
///
/// /login/getLoginQrCode
///
/// # Parameters
///
/// - `app_id` - The application identifier associated with the user.
///
/// # Examples
///
/// ```rust, no_run
/// #[tokio::main]
/// async fn main() {
///     use rgewe_api::api;
///     let app_id = "your_token"; // Application identifier
///     let value = api::get_login_qr(app_id).await.unwrap();
///     // 200 means success, 500 means failure with relogin
///     assert!(value.get("ret").unwrap().as_u64().unwrap() == 200);
/// }
/// ```
pub async fn get_login_qr(app_id: &str) -> Result<Value, Box<dyn Error>> {
    let params = json!({
        "appId": app_id, // Application ID
    });
    util::gewe_post_json("/login/getLoginQrCode", Some(params)).await
}

/// Check login QR code status API
///
/// Wrapper of calling /login/checkLogin API of gewe service.
/// Check the status of the login QR code and the login session.
///
/// # Route
///
/// /login/checkLogin
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
///     let app_id = "your_app_id";  // Application identifier
///     let uuid = "your_uuid";      // UUID of the login session
///     let captcha_code = "";       // Optional captcha code if required
///     let value = api::check_login_qr(app_id, uuid, captcha_code).await.unwrap();
///     assert!(value.get("ret").unwrap().as_u64().unwrap() == 200);
/// }
/// ```
pub async fn check_login_qr(
    app_id: &str,
    uuid: &str,
    captcha_code: &str,
) -> Result<Value, Box<dyn Error>> {
    let params = json!({
        "appId": app_id,         // Application ID
        "uuid": uuid,            // UUID of the login session
        "captchCode": captcha_code, // Captcha code for verification
    });
    util::gewe_post_json("/login/checkLogin", Some(params)).await
}

/// Log out API
///
/// Wrapper of calling /login/logout API of gewe service.
/// Logout the user associated with the given app ID.
///
/// # Route
///
/// /login/logout
///
/// # Examples
///
/// ```rust,no_run
/// #[tokio::main]
/// async fn main() {
///     use rgewe_api::api;
///     let app_id = "your_app_id"; // Application identifier
///     let value = api::log_out(app_id).await.unwrap();
///     assert!(value.get("ret").unwrap().as_u64().unwrap() == 200);
/// }
/// ```
pub async fn log_out(app_id: &str) -> Result<Value, Box<dyn Error>> {
    let params = json!({
        "appId": app_id, // Application ID
    });
    util::gewe_post_json("/login/logout", Some(params)).await
}

/// Dialog-based login API
///
/// Wrapper of calling /login/dialogLogin API of gewe service.
/// TODO, not used in the current version.
///
/// # Route
///
/// /login/dialogLogin
///
/// # Examples
///
/// ```rust,no_run
/// #[tokio::main]
/// async fn main() {
///     use rgewe_api::api;
///     let app_id = "your_app_id"; // Application identifier
///     let value = api::dialog_login(app_id).await.unwrap();
///     assert!(value.get("ret").unwrap().as_u64().unwrap() == 200);
/// }
/// ```
pub async fn dialog_login(app_id: &str) -> Result<Value, Box<dyn Error>> {
    let params = json!({
        "appId": app_id, // Application ID
    });
    util::gewe_post_json("/login/dialogLogin", Some(params)).await
}

/// Check online status API
///
/// Wrapper of calling /login/checkOnline API of gewe service.
/// Checks whether the user with specific `app_id` is currently online.
///
/// # Route
///
/// /login/checkOnline
///
/// # Examples
///
/// ```rust,no_run
/// #[tokio::main]
/// async fn main() {
///     use rgewe_api::api;
///     let app_id = "your_app_id";
///     let value = api::check_online(app_id).await.unwrap();
///     assert!(value.get("ret").unwrap().as_u64().unwrap() == 200);
///     let is_online = value
///         .get("data")
///         .unwrap()
///         .as_bool()
///         .unwrap();
///     assert_eq!(is_online, true)
/// }
/// ```
pub async fn check_online(app_id: &str) -> Result<Value, Box<dyn Error>> {
    let params = json!({
        "appId": app_id, // Application ID
    });
    util::gewe_post_json("/login/checkOnline", Some(params)).await
}
