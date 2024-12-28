use crate::util;
use serde_json::{json, Value};
use std::error::Error;

/// Get user profile API
///
/// Wrapper of calling `/personal/getProfile` API of the service.
/// Get the user profile information (robot profile).
///
/// # Route
///
/// `/personal/getProfile`
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
///     let app_id = "your_app_id";
///     let value = api::get_profile(app_id).await.unwrap();
///     assert!(value.get("ret").unwrap().as_u64().unwrap() == 200);
/// }
/// ```
pub async fn get_profile(app_id: &str) -> Result<Value, Box<dyn Error>> {
    let params = json!({
        "appId": app_id
    });
    util::gewe_post_json("/personal/getProfile", Some(params)).await
}

/// Get personal QR code API
///
/// Wrapper of calling `/personal/getQrCode` API of the service.
/// Get the personal QR code of the user.
/// QR image styple will change each time you call this API.
///
/// # Route
///
/// `/personal/getQrCode`
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
///     let app_id = "your_app_id";
///     let value = api::get_personal_qr(app_id).await.unwrap();
///     assert!(value.get("ret").unwrap().as_u64().unwrap() == 200);
/// }
/// ```
pub async fn get_personal_qr(app_id: &str) -> Result<Value, Box<dyn Error>> {
    let params = json!({
        "appId": app_id
    });
    util::gewe_post_json("/personal/getQrCode", Some(params)).await
}

/// Get service information API
///
/// Wrapper of calling `/personal/getSafetyInfo` API of the service.
/// Get all service information of the user.
///
/// # Route
///
/// `/personal/getSafetyInfo`
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
///     let app_id = "your_app_id";
///     let value = api::get_safety_info(app_id).await.unwrap();
///     assert!(value.get("ret").unwrap().as_u64().unwrap() == 200);
/// }
/// ```
pub async fn get_safety_info(app_id: &str) -> Result<Value, Box<dyn Error>> {
    let params = json!({
        "appId": app_id,
    });
    util::gewe_post_json("/personal/getSafetyInfo", Some(params)).await
}

#[derive(Debug)]
#[repr(u32)]
pub enum PrivacyOperationType {
    /// Require verification to be added as a friend.
    RequiredFriendRequest = 4,
    /// Recommend friends from my contacts.
    FindMobileContacts = 7,
    /// Allow finding me via phone number.
    FindViaPhoneNumber = 8,
    /// Allow finding me via WeChat ID.
    FindViaWeChatId = 25,
    /// Allow finding me via group chat.
    FindViaGroupChat = 38,
    /// Allow adding me via QR code.
    AddViaQrCode = 39,
    /// Allow adding me via business card.
    AddViaBusinessCard = 40,
}

/// Update privacy settings API
///
/// Wrapper of calling `/personal/privacySettings` API of the service.
///
/// # Route
///
/// `/personal/privacySettings`
///
/// # Parameters
///
/// - `app_id` - The application identifier associated with the user.
/// - `option` - The privacy option to be updated. See [`PrivacyOperationType`]
/// - `open` - Whether to enable (`true`) or disable (`false`) the privacy setting.
///
/// # Examples
///
/// ```rust,no_run
/// #[tokio::main]
/// async fn main() {
///     use rgewe_api::api::{self, PrivacyOperationType};
///     let app_id = "your_app_id";
///     let option = PrivacyOperationType::FindMobileContacts;
///     let open = false;
///     let value = api::privacy_settings(app_id, option, open).await.unwrap();
///     assert!(value.get("ret").unwrap().as_u64().unwrap() == 200);
/// }
/// ```
pub async fn privacy_settings(
    app_id: &str,
    option: PrivacyOperationType,
    open: bool,
) -> Result<Value, Box<dyn Error>> {
    let params = json!({
        "appId": app_id,
        "option": option as u32,
        "open": open,
    });
    util::gewe_post_json("/personal/privacySettings", Some(params)).await
}

#[derive(Debug)]
#[repr(u32)]
pub enum Sex {
    Male = 1,
    Female = 2,
}

/// Update user profile API
///
/// Wrapper of calling `/personal/updateProfile` API of the service.
///
/// # Route
///
/// `/personal/updateProfile`
///
/// # Parameters
///
/// - `app_id` - The application identifier associated with the user.
/// - `city` - The city where the user is located.
/// - `country` - The country where the user is located.
/// - `nick_name` - The user's nickname.
/// - `province` - The province where the user is located.
/// - `sex` - The user's gender (`Male` or `Female`).
/// - `signature` - The user's personal signature.
///
/// # Examples
///
/// ```rust, no_run
/// #[tokio::main]
/// async fn main() {
///     use rgewe_api::api::{self, Sex};
///     let app_id = "your_app_id";
///     let city = "Shanghai";
///     let country = "China";
///     let nick_name = "Rustacean";
///     let province = "Shanghai";
///     let sex = Sex::Male;
///     let signature = "Hello, Rust!";
///     let value = api::update_profile(app_id, city, country, nick_name, province, sex, signature).await.unwrap();
///     assert!(value.get("ret").unwrap().as_u64().unwrap() == 200);
/// }
/// ```
pub async fn update_profile(
    app_id: &str,
    city: &str,
    country: &str,
    nick_name: &str,
    province: &str,
    sex: Sex,
    signature: &str,
) -> Result<Value, Box<dyn Error>> {
    let params = json!({
        "appId": app_id,
        "city": city,
        "country": country,
        "nickName": nick_name,
        "province": province,
        "sex": sex as u32,
        "signature": signature,
    });
    util::gewe_post_json("/personal/updateProfile", Some(params)).await
}

pub async fn update_head_img(app_id: &str, head_img_url: &str) -> Result<Value, Box<dyn Error>> {
    let params = json!({
        "appId": app_id,
        "headImgUrl": head_img_url,
    });
    util::gewe_post_json("/personal/updateHeadImg", Some(params)).await
}
