use serde_json::{json, Value};
use std::error::Error;

use super::{ApiClient, PrivacyOperationType, Sex};

impl ApiClient {
    impl_params_api!(
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
    ///     use rgewe::api;
    ///     let app_id = "your_app_id";
    ///     let value = api::get_profile(app_id).await.unwrap();
    ///     assert!(value.get("ret").unwrap().as_u64().unwrap() == 200);
    /// }
    /// ```
    ///
    get_profile,
    "/personal/getProfile",
    ("appId", app_id, &str));
    // v0.1.0
    // pub async fn get_profile(app_id: &str) -> Result<Value, Box<dyn Error>> {
    //     let params = json!({
    //         "appId": app_id
    //     });
    //     util::gewe_post_json("/personal/getProfile", Some(params)).await
    // }

    impl_params_api!(
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
    ///     use rgewe::api;
    ///     let app_id = "your_app_id";
    ///     let value = api::get_personal_qr(app_id).await.unwrap();
    ///     assert!(value.get("ret").unwrap().as_u64().unwrap() == 200);
    /// }
    /// ```
    get_personal_qr,
    "/personal/getQrCode",
    ("appId", app_id, &str));
    // v0.1.0
    // pub async fn get_personal_qr(app_id: &str) -> Result<Value, Box<dyn Error>> {
    //     let params = json!({
    //         "appId": app_id
    //     });
    //     util::gewe_post_json("/personal/getQrCode", Some(params)).await
    // }

    impl_params_api!(
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
    ///     use rgewe::api;
    ///     let app_id = "your_app_id";
    ///     let value = api::get_safety_info(app_id).await.unwrap();
    ///     assert!(value.get("ret").unwrap().as_u64().unwrap() == 200);
    /// }
    /// ```
    get_safety_info,
    "/personal/getSafetyInfo",
    ("appId", app_id, &str));
    // v0.1.0
    // pub async fn get_safety_info(app_id: &str) -> Result<Value, Box<dyn Error>> {
    //     let params = json!({
    //         "appId": app_id,
    //     });
    //     util::gewe_post_json("/personal/getSafetyInfo", Some(params)).await
    // }

    impl_params_api!(
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
    ///     use rgewe::api::{self, PrivacyOperationType};
    ///     let app_id = "your_app_id";
    ///     let option = PrivacyOperationType::FindMobileContacts;
    ///     let open = false;
    ///     let value = api::privacy_settings(app_id, option, open).await.unwrap();
    ///     assert!(value.get("ret").unwrap().as_u64().unwrap() == 200);
    /// }
    /// ```
    privacy_settings,
    "/personal/privacySettings",
    ("appId", app_id, &str),
    ("option", option, PrivacyOperationType),
    ("open", open, bool));
    // v0.1.0
    // pub async fn privacy_settings(
    //     app_id: &str,
    //     option: PrivacyOperationType,
    //     open: bool,
    // ) -> Result<Value, Box<dyn Error>> {
    //     let params = json!({
    //         "appId": app_id,
    //         "option": option as u32,
    //         "open": open,
    //     });
    //     util::gewe_post_json("/personal/privacySettings", Some(params)).await
    // }

    impl_params_api!(
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
    ///     use rgewe::api::{self, Sex};
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
    update_profile,
    "/personal/updateProfile",
    ("appId", app_id, &str),
    ("city", city, &str),
    ("country", country, &str),
    ("nickName", nick_name, &str),
    ("province", province, &str),
    ("sex", sex, Sex),
    ("signature", signature, &str));
    // v0.1.0
    // pub async fn update_profile(
    //     app_id: &str,
    //     city: &str,
    //     country: &str,
    //     nick_name: &str,
    //     province: &str,
    //     sex: Sex,
    //     signature: &str,
    // ) -> Result<Value, Box<dyn Error>> {
    //     let params = json!({
    //         "appId": app_id,
    //         "city": city,
    //         "country": country,
    //         "nickName": nick_name,
    //         "province": province,
    //         "sex": sex as u32,
    //         "signature": signature,
    //     });
    //     util::gewe_post_json("/personal/updateProfile", Some(params)).await
    // }

    impl_params_api!(
    /// Update user head image API
    /// TODO: need add doc
    update_head_img,
    "/personal/updateHeadImg",
    ("appId", app_id, &str),
    ("headImgUrl", head_img_url, &str));
    // v0.1.0
    // pub async fn update_head_img(app_id: &str, head_img_url: &str) -> Result<Value, Box<dyn Error>> {
    //     let params = json!({
    //         "appId": app_id,
    //         "headImgUrl": head_img_url,
    //     });
    //     util::gewe_post_json("/personal/updateHeadImg", Some(params)).await
    // }
}
