#![allow(clippy::too_many_arguments)]
use reqwest::{header::HeaderMap, Client};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::error::Error;

const BASE_URL: &str = "http://localhost:2531/v2/api";
const HEADER_GEWE: &str = "X-GEWE-TOKEN";
const _HEADER_SELF: &str = "X-GEWE-RGEWE";

pub async fn test_get_ip() -> Result<(), Box<dyn Error>> {
    let resp = reqwest::get("https://httpbin.org/ip")
        .await?
        .json::<std::collections::HashMap<String, String>>()
        .await?;
    println!("[test_get_ip] show response:");
    for (k, v) in resp.iter() {
        println!("{}: {}", k, v);
    }
    Ok(())
}

async fn post_json(
    url: &str,
    with_proxy: bool,
    headers: HeaderMap,
    body: &Option<Value>,
) -> Result<Value, Box<dyn Error>> {
    let client = if with_proxy {
        // reqwest client use system proxy as default
        Client::new()
    } else {
        Client::builder().no_proxy().build().unwrap()
    };

    let rest = client
        .post(url)
        // TODO: change headers to ref?
        .headers(headers)
        .json(body)
        .send()
        .await?
        .json()
        .await?;
    Ok(rest)
}

#[derive(Debug, Default)]
pub struct ApiClient {
    pub token: String,
    pub base_url: String,
}

pub struct ApiClientBuilder {
    token: Option<String>,
    base_url: Option<String>,
}
impl Default for ApiClientBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl ApiClientBuilder {
    pub fn new() -> Self {
        Self {
            token: None,
            base_url: None,
        }
    }
    pub fn with_token(mut self, token: &str) -> Self {
        self.token = Some(token.to_string());
        self
    }
    pub fn with_base_url(mut self, base_url: &str) -> Self {
        self.base_url = Some(base_url.to_string());
        self
    }
    pub fn build(self) -> ApiClient {
        ApiClient {
            token: self.token.unwrap_or_default(),
            base_url: self.base_url.unwrap_or_else(|| BASE_URL.to_string()),
        }
    }
}

impl ApiClient {
    // pub fn new() -> Self {
    //     Self::default()
    // }
    pub async fn gewe_post_json(
        &self,
        route: &str,
        body: Option<Value>,
    ) -> Result<Value, Box<dyn Error>> {
        let mut headers = HeaderMap::new();
        headers.insert(HEADER_GEWE, self.token.parse().unwrap());

        // Check json body
        if body.is_none() {
            // Empty json body
            // only allow ask for token
            if route != "/tools/getTokenId" {
                return Err(format!("Empty json body for route: {}", route).into());
            }
        }
        let url = format!("{}{}", BASE_URL, route);
        let with_proxy = false;
        // TODO: add proxy configuration
        post_json(&url, with_proxy, headers, &body).await
    }
}

/// Represents a WeChat ID (Wxid) that must start with "wxid_" and be within a valid length range.
/// - Unnamed single-field struct → Serialized directly as the field’s value (used here)
/// - Unnamed multi-field struct → Serialized as a JSON array
/// - Named multi-field struct → Serialized as a JSON object
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Wxid(String);

impl Wxid {
    /// Returns the Wxid as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<&str> for Wxid {
    type Error = String;

    /// Attempts to create a Wxid from a string slice.
    ///
    /// Fails if the string does not start with "wxid_" or is not within the valid length range.
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.starts_with("wxid_") || value.ends_with("@chatroom") {
            // TODO value.len() > 5 && value.len() < 50
            Ok(Wxid(value.to_string()))
        } else {
            Err(format!("Invalid Wxid format: {}", value))
        }
    }
}

impl std::fmt::Display for Wxid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

macro_rules! impl_params_api {
  ($(#[$meta:meta])* $f_name:ident, $route:expr, $(($p_k:expr, $p_v:ident, $p_t:ty)),* $(,)? ) => {
      $(#[$meta])*
      pub async fn $f_name(&self, $($p_v: $p_t),*) -> Result<Value, Box<dyn Error>> {
          let p = json!({
            $($p_k: $p_v),*
          });
          self.gewe_post_json($route, Some(p)).await
      }
  };
}

#[derive(Debug, Deserialize, Serialize)]
#[repr(u32)]
pub enum Sex {
    Male = 1,
    Female = 2,
}

#[derive(Debug, Deserialize, Serialize)]
#[repr(u32)]
pub enum ContactOperationType {
    Add = 1,
    Remove = 2,
}

#[derive(Debug, Deserialize, Serialize)]
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

pub mod contacts_api;
pub mod favor_api;
pub mod group_api;
pub mod label_api;
pub mod login_api;
pub mod message_api;
pub mod personal_api;
