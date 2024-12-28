#![allow(clippy::too_many_arguments)]
use crate::user::Wxid;
use crate::util;
use serde_json::json;
use serde_json::Value;
use std::error::Error;

pub async fn post_text(
    app_id: &str,
    to_wxid: &Wxid,
    content: &str,
    ats: &str,
) -> Result<Value, Box<dyn Error>> {
    // POST /message/postText
    let params = json!({
        "appId": app_id,
        "toWxid": to_wxid,
        "content": content,
        "ats": ats
    });
    util::gewe_post_json("/message/postText", Some(params)).await
}

/// Send a file message
/// POST /message/postFile
pub async fn post_file(
    app_id: &str,
    to_wxid: &Wxid,
    file_url: &str,
    file_name: &str,
) -> Result<Value, Box<dyn Error>> {
    let params = json!({
        "appId": app_id,         // Application ID
        "toWxid": to_wxid,       // Target WeChat ID
        "fileUrl": file_url,     // URL of the file
        "fileName": file_name    // File name to display
    });
    util::gewe_post_json("/message/postFile", Some(params)).await
}

/// Send an image message
/// POST /message/postImage
pub async fn post_image(
    app_id: &str,
    to_wxid: &Wxid,
    img_url: &str,
) -> Result<Value, Box<dyn Error>> {
    let params = json!({
        "appId": app_id,         // Application ID
        "toWxid": to_wxid,       // Target WeChat ID
        "imgUrl": img_url        // URL of the image
    });
    util::gewe_post_json("/message/postImage", Some(params)).await
}

/// Send a voice message
/// POST /message/postVoice
pub async fn post_voice(
    app_id: &str,
    to_wxid: &Wxid,
    voice_url: &str,
    voice_duration: u32,
) -> Result<Value, Box<dyn Error>> {
    let params = json!({
        "appId": app_id,             // Application ID
        "toWxid": to_wxid,           // Target WeChat ID
        "voiceUrl": voice_url,       // URL of the voice file
        "voiceDuration": voice_duration // Duration of the voice in seconds
    });
    util::gewe_post_json("/message/postVoice", Some(params)).await
}

/// Send a video message
/// POST /message/postVideo
pub async fn post_video(
    app_id: &str,
    to_wxid: &Wxid,
    video_url: &str,
    thumb_url: &str,
    video_duration: u32,
) -> Result<Value, Box<dyn Error>> {
    let params = json!({
        "appId": app_id,             // Application ID
        "toWxid": to_wxid,           // Target WeChat ID
        "videoUrl": video_url,       // URL of the video file
        "thumbUrl": thumb_url,       // URL of the video thumbnail
        "videoDuration": video_duration // Duration of the video in seconds
    });
    util::gewe_post_json("/message/postVideo", Some(params)).await
}

/// Send a link message
/// POST /message/postLink
pub async fn post_link(
    app_id: &str,
    to_wxid: &Wxid,
    title: &str,
    desc: &str,
    link_url: &str,
    thumb_url: &str,
) -> Result<Value, Box<dyn Error>> {
    let params = json!({
        "appId": app_id,         // Application ID
        "toWxid": to_wxid,       // Target WeChat ID
        "title": title,          // Title of the link
        "desc": desc,            // Description of the link
        "linkUrl": link_url,     // URL of the link
        "thumbUrl": thumb_url    // Thumbnail image URL for the link
    });
    util::gewe_post_json("/message/postLink", Some(params)).await
}

/// Send a name card message
/// POST /message/postNameCard
pub async fn post_name_card(
    app_id: &str,
    to_wxid: &Wxid,
    nick_name: &str,
    name_card_wxid: &str,
) -> Result<Value, Box<dyn Error>> {
    let params = json!({
        "appId": app_id,         // Application ID
        "toWxid": to_wxid,       // Target WeChat ID
        "nickName": nick_name,   // Nickname of the contact
        "nameCardWxid": name_card_wxid // WeChat ID of the contact's name card
    });
    util::gewe_post_json("/message/postNameCard", Some(params)).await
}

/// Send an emoji message
/// POST /message/postEmoji
pub async fn post_emoji(
    app_id: &str,
    to_wxid: &Wxid,
    emoji_md5: &str,
    emoji_size: &str,
) -> Result<Value, Box<dyn Error>> {
    let params = json!({
        "appId": app_id,         // Application ID
        "toWxid": to_wxid,       // Target WeChat ID
        "emojiMd5": emoji_md5,   // MD5 hash of the emoji
        "emojiSize": emoji_size  // Size of the emoji
    });
    util::gewe_post_json("/message/postEmoji", Some(params)).await
}

/// Send an app message
/// POST /message/postAppMsg
pub async fn post_app_msg(
    app_id: &str,
    to_wxid: &Wxid,
    appmsg: &str,
) -> Result<Value, Box<dyn Error>> {
    let params = json!({
        "appId": app_id,         // Application ID
        "toWxid": to_wxid,       // Target WeChat ID
        "appmsg": appmsg         // Content of the app message
    });
    util::gewe_post_json("/message/postAppMsg", Some(params)).await
}

/// Send a mini app message
/// POST /message/postMiniApp
pub async fn post_mini_app(
    app_id: &str,
    to_wxid: &Wxid,
    mini_app_id: &str,
    display_name: &str,
    page_path: &str,
    cover_img_url: &str,
    title: &str,
    user_name: &str,
) -> Result<Value, Box<dyn Error>> {
    let params = json!({
        "appId": app_id,             // Application ID
        "toWxid": to_wxid,           // Target WeChat ID
        "miniAppId": mini_app_id,    // Mini app ID
        "displayName": display_name, // Display name of the mini app
        "pagePath": page_path,       // Path within the mini app
        "coverImgUrl": cover_img_url,// Cover image URL for the mini app
        "title": title,              // Title of the mini app
        "userName": user_name        // Username associated with the mini app
    });
    util::gewe_post_json("/message/postMiniApp", Some(params)).await
}

/// Forward a file API
pub async fn forward_file(
    app_id: &str,
    to_wxid: &Wxid,
    xml: &str,
) -> Result<Value, Box<dyn Error>> {
    let params = json!({
        "appId": app_id,
        "toWxid": to_wxid,
        "xml": xml,
    });
    util::gewe_post_json("/message/forwardFile", Some(params)).await
}

/// Forward an image API
pub async fn forward_image(
    app_id: &str,
    to_wxid: &Wxid,
    xml: &str,
) -> Result<Value, Box<dyn Error>> {
    let params = json!({
        "appId": app_id,
        "toWxid": to_wxid,
        "xml": xml,
    });
    util::gewe_post_json("/message/forwardImage", Some(params)).await
}

/// Forward a video API
pub async fn forward_video(
    app_id: &str,
    to_wxid: &Wxid,
    xml: &str,
) -> Result<Value, Box<dyn Error>> {
    let params = json!({
        "appId": app_id,
        "toWxid": to_wxid,
        "xml": xml,
    });
    util::gewe_post_json("/message/forwardVideo", Some(params)).await
}

/// Forward a URL API
pub async fn forward_url(app_id: &str, to_wxid: &Wxid, xml: &str) -> Result<Value, Box<dyn Error>> {
    let params = json!({
        "appId": app_id,
        "toWxid": to_wxid,
        "xml": xml,
    });
    util::gewe_post_json("/message/forwardUrl", Some(params)).await
}

/// Forward a mini-app API
pub async fn forward_mini_app(
    app_id: &str,
    to_wxid: &Wxid,
    xml: &str,
    cover_img_url: &str,
) -> Result<Value, Box<dyn Error>> {
    let params = json!({
        "appId": app_id,
        "toWxid": to_wxid,
        "xml": xml,
        "coverImgUrl": cover_img_url,
    });
    util::gewe_post_json("/message/forwardMiniApp", Some(params)).await
}

/// Revoke a message
pub async fn revoke_msg(
    app_id: &str,
    to_wxid: &Wxid,
    msg_id: &str,
    new_msg_id: &str,
    create_time: &str,
) -> Result<Value, Box<dyn Error>> {
    let params = json!({
        "appId": app_id,             // Application ID
        "toWxid": to_wxid,           // Target WeChat ID
        "msgId": msg_id,             // Original message ID
        "newMsgId": new_msg_id,      // New message ID for the replacement
        "createTime": create_time    // Creation time of the original message
    });
    util::gewe_post_json("/message/revokeMsg", Some(params)).await
}
