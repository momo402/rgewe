#![allow(clippy::too_many_arguments)]
use serde_json::{json, Value};
use std::error::Error;

use super::{ApiClient, Wxid};

impl ApiClient {
    impl_params_api!(
    /// Send a text message
    ///
    /// Wrapper of calling `/message/postText` API of the service.
    /// Send a text message with optional `@` mentions (ats) to a specified id (person / group).
    ///
    /// # Route
    ///
    /// `/message/postText`
    ///
    /// # Parameters
    ///
    /// - `app_id` - The application identifier associated with the user.
    /// - `to_wxid` - The target WeChat ID to send the message to.
    /// - `content` - The content of the text message.
    /// - `ats` - Optional mentions (e.g., "@username"). `notify@all` for ats all
    ///
    /// # Examples
    ///
    /// TODO
    post_text,
    "/message/postText",
    ("appId", app_id, &str),
    ("toWxid", to_wxid, &Wxid),
    ("content", content, &str),
    ("ats", ats, &str));
    // v0.1.0
    // pub async fn post_text(
    //     app_id: &str,
    //     to_wxid: &Wxid,
    //     content: &str,
    //     ats: &str,
    // ) -> Result<Value, Box<dyn Error>> {
    //     // POST /message/postText
    //     let params = json!({
    //         "appId": app_id,
    //         "toWxid": to_wxid,
    //         "content": content,
    //         "ats": ats
    //     });
    //     util::gewe_post_json("/message/postText", Some(params)).await
    // }

    impl_params_api!(
    /// Send a file message
    /// POST /message/postFile
    /// TODO: need add doc
    post_file,
    "/message/postFile",
    ("appId", app_id, &str),
    ("toWxid", to_wxid, &Wxid),
    ("fileUrl", file_url, &str),
    ("fileName", file_name, &str));
    // v0.1.0
    // pub async fn post_file(
    //     app_id: &str,
    //     to_wxid: &Wxid,
    //     file_url: &str,
    //     file_name: &str,
    // ) -> Result<Value, Box<dyn Error>> {
    //     let params = json!({
    //         "appId": app_id,         // Application ID
    //         "toWxid": to_wxid,       // Target WeChat ID
    //         "fileUrl": file_url,     // URL of the file
    //         "fileName": file_name    // File name to display
    //     });
    //     util::gewe_post_json("/message/postFile", Some(params)).await
    // }

    impl_params_api!(
    /// Send an image message
    /// POST /message/postImage
    /// TODO: need add doc
    post_image,
    "/message/postImage",
    ("appId", app_id, &str),
    ("toWxid", to_wxid, &Wxid),
    ("imgUrl", img_url, &str));
    // v0.1.0
    // pub async fn post_image(
    //     app_id: &str,
    //     to_wxid: &Wxid,
    //     img_url: &str,
    // ) -> Result<Value, Box<dyn Error>> {
    //     let params = json!({
    //         "appId": app_id,         // Application ID
    //         "toWxid": to_wxid,       // Target WeChat ID
    //         "imgUrl": img_url        // URL of the image
    //     });
    //     util::gewe_post_json("/message/postImage", Some(params)).await
    // }

    impl_params_api!(
    /// Send a voice message
    /// POST /message/postVoice
    /// TODO: need add doc
    post_voice,
    "/message/postVoice",
    ("appId", app_id, &str),
    ("toWxid", to_wxid, &Wxid),
    ("voiceUrl", voice_url, &str),
    ("voiceDuration", voice_duration, u32));
    // v0.1.0
    // pub async fn post_voice(
    //     app_id: &str,
    //     to_wxid: &Wxid,
    //     voice_url: &str,
    //     voice_duration: u32,
    // ) -> Result<Value, Box<dyn Error>> {
    //     let params = json!({
    //         "appId": app_id,             // Application ID
    //         "toWxid": to_wxid,           // Target WeChat ID
    //         "voiceUrl": voice_url,       // URL of the voice file
    //         "voiceDuration": voice_duration // Duration of the voice in seconds
    //     });
    //     util::gewe_post_json("/message/postVoice", Some(params)).await
    // }

    impl_params_api!(
    /// Send a video message
    /// POST /message/postVideo
    /// TODO: need add doc
    post_video,
    "/message/postVideo",
    ("appId", app_id, &str),
    ("toWxid", to_wxid, &Wxid),
    ("videoUrl", video_url, &str),
    ("thumbUrl", thumb_url, &str),
    ("videoDuration", video_duration, u32));
    // v0.1.0
    // pub async fn post_video(
    //     app_id: &str,
    //     to_wxid: &Wxid,
    //     video_url: &str,
    //     thumb_url: &str,
    //     video_duration: u32,
    // ) -> Result<Value, Box<dyn Error>> {
    //     let params = json!({
    //         "appId": app_id,             // Application ID
    //         "toWxid": to_wxid,           // Target WeChat ID
    //         "videoUrl": video_url,       // URL of the video file
    //         "thumbUrl": thumb_url,       // URL of the video thumbnail
    //         "videoDuration": video_duration // Duration of the video in seconds
    //     });
    //     util::gewe_post_json("/message/postVideo", Some(params)).await
    // }

    impl_params_api!(
    /// Send a link message
    /// POST /message/postLink
    /// TODO: need add doc
    post_link,
    "/message/postLink",
    ("appId", app_id, &str),
    ("toWxid", to_wxid, &Wxid),
    ("title", title, &str),
    ("desc", desc, &str),
    ("linkUrl", link_url, &str),
    ("thumbUrl", thumb_url, &str));
    // v0.1.0
    // pub async fn post_link(
    //     app_id: &str,
    //     to_wxid: &Wxid,
    //     title: &str,
    //     desc: &str,
    //     link_url: &str,
    //     thumb_url: &str,
    // ) -> Result<Value, Box<dyn Error>> {
    //     let params = json!({
    //         "appId": app_id,         // Application ID
    //         "toWxid": to_wxid,       // Target WeChat ID
    //         "title": title,          // Title of the link
    //         "desc": desc,            // Description of the link
    //         "linkUrl": link_url,     // URL of the link
    //         "thumbUrl": thumb_url    // Thumbnail image URL for the link
    //     });
    //     util::gewe_post_json("/message/postLink", Some(params)).await
    // }

    impl_params_api!(
    /// Send a name card message
    /// POST /message/postNameCard
    /// TODO: need add doc
    post_name_card,
    "/message/postNameCard",
    ("appId", app_id, &str),
    ("toWxid", to_wxid, &Wxid),
    ("nickName", nick_name, &str),
    ("nameCardWxid", name_card_wxid, &str));
    // v0.1.0
    // pub async fn post_name_card(
    //     app_id: &str,
    //     to_wxid: &Wxid,
    //     nick_name: &str,
    //     name_card_wxid: &str,
    // ) -> Result<Value, Box<dyn Error>> {
    //     let params = json!({
    //         "appId": app_id,         // Application ID
    //         "toWxid": to_wxid,       // Target WeChat ID
    //         "nickName": nick_name,   // Nickname of the contact
    //         "nameCardWxid": name_card_wxid // WeChat ID of the contact's name card
    //     });
    //     util::gewe_post_json("/message/postNameCard", Some(params)).await
    // }

    impl_params_api!(
    /// Send an emoji message
    /// POST /message/postEmoji
    /// TODO: need add doc
    post_emoji,
    "/message/postEmoji",
    ("appId", app_id, &str),
    ("toWxid", to_wxid, &Wxid),
    ("emojiMd5", emoji_md5, &str),
    ("emojiSize", emoji_size, &str));
    // v0.1.0
    // pub async fn post_emoji(
    //     app_id: &str,
    //     to_wxid: &Wxid,
    //     emoji_md5: &str,
    //     emoji_size: &str,
    // ) -> Result<Value, Box<dyn Error>> {
    //     let params = json!({
    //         "appId": app_id,         // Application ID
    //         "toWxid": to_wxid,       // Target WeChat ID
    //         "emojiMd5": emoji_md5,   // MD5 hash of the emoji
    //         "emojiSize": emoji_size  // Size of the emoji
    //     });
    //     util::gewe_post_json("/message/postEmoji", Some(params)).await
    // }

    impl_params_api!(
    /// Send an app message
    /// POST /message/postAppMsg
    /// TODO: need add doc
    post_app_msg,
    "/message/postAppMsg",
    ("appId", app_id, &str),
    ("toWxid", to_wxid, &Wxid),
    ("appmsg", appmsg, &str));
    // v0.1.0
    // pub async fn post_app_msg(
    //     app_id: &str,
    //     to_wxid: &Wxid,
    //     appmsg: &str,
    // ) -> Result<Value, Box<dyn Error>> {
    //     let params = json!({
    //         "appId": app_id,         // Application ID
    //         "toWxid": to_wxid,       // Target WeChat ID
    //         "appmsg": appmsg         // Content of the app message
    //     });
    //     util::gewe_post_json("/message/postAppMsg", Some(params)).await
    // }

    impl_params_api!(
    /// Send a mini app message
    /// POST /message/postMiniApp
    /// TODO: need add doc
    post_mini_app,
    "/message/postMiniApp",
    ("appId", app_id, &str),
    ("toWxid", to_wxid, &Wxid),
    ("miniAppId", mini_app_id, &str),
    ("displayName", display_name, &str),
    ("pagePath", page_path, &str),
    ("coverImgUrl", cover_img_url, &str),
    ("title", title, &str),
    ("userName", user_name, &str));
    // v0.1.0
    // pub async fn post_mini_app(
    //     app_id: &str,
    //     to_wxid: &Wxid,
    //     mini_app_id: &str,
    //     display_name: &str,
    //     page_path: &str,
    //     cover_img_url: &str,
    //     title: &str,
    //     user_name: &str,
    // ) -> Result<Value, Box<dyn Error>> {
    //     let params = json!({
    //         "appId": app_id,             // Application ID
    //         "toWxid": to_wxid,           // Target WeChat ID
    //         "miniAppId": mini_app_id,    // Mini app ID
    //         "displayName": display_name, // Display name of the mini app
    //         "pagePath": page_path,       // Path within the mini app
    //         "coverImgUrl": cover_img_url,// Cover image URL for the mini app
    //         "title": title,              // Title of the mini app
    //         "userName": user_name        // Username associated with the mini app
    //     });
    //     util::gewe_post_json("/message/postMiniApp", Some(params)).await
    // }

    impl_params_api!(
    /// Forward a file API
    /// TODO: need add doc
    forward_file,
    "/message/forwardFile",
    ("appId", app_id, &str),
    ("toWxid", to_wxid, &Wxid),
    ("xml", xml, &str));
    // v0.1.0
    // pub async fn forward_file(
    //     app_id: &str,
    //     to_wxid: &Wxid,
    //     xml: &str,
    // ) -> Result<Value, Box<dyn Error>> {
    //     let params = json!({
    //         "appId": app_id,
    //         "toWxid": to_wxid,
    //         "xml": xml,
    //     });
    //     util::gewe_post_json("/message/forwardFile", Some(params)).await
    // }

    impl_params_api!(
    /// Forward an image API
    /// TODO: need add doc
    forward_image,
    "/message/forwardImage",
    ("appId", app_id, &str),
    ("toWxid", to_wxid, &Wxid),
    ("xml", xml, &str));
    // v0.1.0
    // pub async fn forward_image(
    //     app_id: &str,
    //     to_wxid: &Wxid,
    //     xml: &str,
    // ) -> Result<Value, Box<dyn Error>> {
    //     let params = json!({
    //         "appId": app_id,
    //         "toWxid": to_wxid,
    //         "xml": xml,
    //     });
    //     util::gewe_post_json("/message/forwardImage", Some(params)).await
    // }

    impl_params_api!(
    /// Forward a video API
    /// TODO: need add doc
    forward_video,
    "/message/forwardVideo",
    ("appId", app_id, &str),
    ("toWxid", to_wxid, &Wxid),
    ("xml", xml, &str));
    // v0.1.0
    // pub async fn forward_video(
    //     app_id: &str,
    //     to_wxid: &Wxid,
    //     xml: &str,
    // ) -> Result<Value, Box<dyn Error>> {
    //     let params = json!({
    //         "appId": app_id,
    //         "toWxid": to_wxid,
    //         "xml": xml,
    //     });
    //     util::gewe_post_json("/message/forwardVideo", Some(params)).await
    // }

    impl_params_api!(
    /// Forward a URL API
    /// TODO: need add doc
    forward_url,
    "/message/forwardUrl",
    ("appId", app_id, &str),
    ("toWxid", to_wxid, &Wxid),
    ("xml", xml, &str));
    // v0.1.0
    // pub async fn forward_url(
    //     app_id: &str,
    //     to_wxid: &Wxid,
    //     xml: &str,
    // ) -> Result<Value, Box<dyn Error>> {
    //     let params = json!({
    //         "appId": app_id,
    //         "toWxid": to_wxid,
    //         "xml": xml,
    //     });
    //     util::gewe_post_json("/message/forwardUrl", Some(params)).await
    // }

    impl_params_api!(
    /// Forward a mini-app API
    /// TODO: need add doc
    forward_mini_app,
    "/message/forwardMiniApp",
    ("appId", app_id, &str),
    ("toWxid", to_wxid, &Wxid),
    ("xml", xml, &str),
    ("coverImgUrl", cover_img_url, &str));
    // v0.1.0
    // pub async fn forward_mini_app(
    //     app_id: &str,
    //     to_wxid: &Wxid,
    //     xml: &str,
    //     cover_img_url: &str,
    // ) -> Result<Value, Box<dyn Error>> {
    //     let params = json!({
    //         "appId": app_id,
    //         "toWxid": to_wxid,
    //         "xml": xml,
    //         "coverImgUrl": cover_img_url,
    //     });
    //     util::gewe_post_json("/message/forwardMiniApp", Some(params)).await
    // }

    impl_params_api!(
    /// Revoke a message
    /// TODO: need add doc
    revoke_msg,
    "/message/revokeMsg",
    ("appId", app_id, &str),
    ("toWxid", to_wxid, &Wxid),
    ("msgId", msg_id, &str),
    ("newMsgId", new_msg_id, &str),
    ("createTime", create_time, &str));
    // v0.1.0
    // pub async fn revoke_msg(
    //     app_id: &str,
    //     to_wxid: &Wxid,
    //     msg_id: &str,
    //     new_msg_id: &str,
    //     create_time: &str,
    // ) -> Result<Value, Box<dyn Error>> {
    //     let params = json!({
    //         "appId": app_id,             // Application ID
    //         "toWxid": to_wxid,           // Target WeChat ID
    //         "msgId": msg_id,             // Original message ID
    //         "newMsgId": new_msg_id,      // New message ID for the replacement
    //         "createTime": create_time    // Creation time of the original message
    //     });
    //     util::gewe_post_json("/message/revokeMsg", Some(params)).await
    // }
}
