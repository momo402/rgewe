use serde_json::{json, Value};
use std::error::Error;

use super::{ApiClient, Wxid};

impl ApiClient {
    impl_params_api!(
    /// Ctreate chatroom/group with at least two users
    /// TODO: need add doc
    create_chatroom,
    "/group/createChatroom",
    ("appId", app_id, &str),
    ("wxids", wxids, Vec<Wxid>));
    // v0.1.0
    // pub async fn create_chatroom(app_id: &str, wxids: Vec<Wxid>) -> Result<Value, Box<dyn Error>> {
    //     let params = json!({
    //         "appId": app_id,
    //         "wxids": wxids,
    //     });
    //     util::gewe_post_json("/group/createChatroom", Some(params)).await
    // }

    impl_params_api!(
    /// Modify chatroom/group name
    /// TODO: need add doc
    modify_chatroom_name,
    "/group/modifyChatroomName",
    ("appId", app_id, &str),
    ("chatroomName", chatroom_name, &str),
    ("chatroomId", chatroom_id, &str));
    // v0.1.0
    // pub async fn modify_chatroom_name(
    //     app_id: &str,
    //     chatroom_name: &str,
    //     chatroom_id: &str,
    // ) -> Result<Value, Box<dyn Error>> {
    //     let params = json!({
    //         "appId": app_id,
    //         "chatroomName": chatroom_name,
    //         "chatroomId": chatroom_id,
    //     });
    //     util::gewe_post_json("/group/modifyChatroomName", Some(params)).await
    // }

    impl_params_api!(
    /// Modify chatroom/group remark (local)
    /// TODO: need add doc
    modify_chatroom_remark,
    "/group/modifyChatroomRemark",
    ("appId", app_id, &str),
    ("chatroomRemark", chatroom_remark, &str),
    ("chatroomId", chatroom_id, &str));
    // v0.1.0
    // pub async fn modify_chatroom_remark(
    //     app_id: &str,
    //     chatroom_remark: &str,
    //     chatroom_id: &str,
    // ) -> Result<Value, Box<dyn Error>> {
    //     let params = json!({
    //         "appId": app_id,
    //         "chatroomRemark": chatroom_remark,
    //         "chatroomId": chatroom_id,
    //     });
    //     util::gewe_post_json("/group/modifyChatroomRemark", Some(params)).await
    // }

    impl_params_api!(
    /// Modify chatroom/group nickname
    /// TODO: need add doc
    modify_chatroom_nickname,
    "/group/modifyChatroomNickNameForSelf",
    ("appId", app_id, &str),
    ("nickName", nick_name, &str),
    ("chatroomId", chatroom_id, &str));
    // v0.1.0
    // pub async fn modify_chatroom_nickname(
    //     app_id: &str,
    //     nick_name: &str,
    //     chatroom_id: &str,
    // ) -> Result<Value, Box<dyn Error>> {
    //     let params = json!({
    //         "appId": app_id,
    //         "nickName": nick_name,
    //         "chatroomId": chatroom_id,
    //     });
    //     util::gewe_post_json("/group/modifyChatroomNickNameForSelf", Some(params)).await
    // }

    impl_params_api!(
    /// Invite new chatroom/group members
    /// TODO: need add doc
    invite_member,
    "/group/inviteMember",
    ("appId", app_id, &str),
    ("wxids", wxids, Vec<Wxid>),
    ("chatroomId", chatroom_id, &str),
    ("reason", reason, &str));
    // v0.1.0
    // pub async fn invite_member(
    //     app_id: &str,
    //     wxids: Vec<Wxid>,
    //     chatroom_id: &str,
    //     reason: &str,
    // ) -> Result<Value, Box<dyn Error>> {
    //     let params = json!({
    //         "appId": app_id,
    //         "wxids": wxids,
    //         "chatroomId": chatroom_id,
    //         "reason": reason,
    //     });
    //     util::gewe_post_json("/group/inviteMember", Some(params)).await
    // }

    impl_params_api!(
    /// Remove members
    /// **Version History:**
    /// - **v0.2.0:**
    ///     - Uses the macro `impl_params_api!` for streamlined implementation.
    ///     - Accepts a pre-flattened string of `wxids` separated by commas (e.g., "wxid_123,wxid_223,wxid_323").
    ///     - Simplifies input to match the original API format and the [`impl_params_api!`] macro.
    /// - **v0.1.0:**
    ///     - Accepts a `Vec<Wxid>` for `wxids` and internally transformed it into a comma-separated string.
    ///     - Requires additional processing to flatten `wxids`
    /// TODO: need add doc
    remove_member,
    "/group/removeMember",
    ("appId", app_id, &str),
    ("wxids", flatten_wxids, &str),
    ("chatroomId", chatroom_id, &str));
    // v0.1.0
    // pub async fn remove_member(
    //     app_id: &str,
    //     wxids: Vec<Wxid>,
    //     chatroom_id: &str,
    // ) -> Result<Value, Box<dyn Error>> {
    //     // TODO double check
    //     let flatten_wxids = wxids
    //         .iter()
    //         .map(|wxid| wxid.to_string())
    //         .collect::<Vec<String>>()
    //         .join(",")
    //     let params = json!({
    //         "appId": app_id,
    //         "wxids": flatten_wxids,
    //         "chatroomId": chatroom_id,
    //     });
    //     util::gewe_post_json("/group/removeMember", Some(params)).await
    // }

    impl_params_api!(
    /// Quit chatroom/group
    /// TODO: need add doc
    quit_chatroom,
    "/group/quitChatroom",
    ("appId", app_id, &str),
    ("chatroomId", chatroom_id, &str));
    // v0.1.0
    // pub async fn quit_chatroom(app_id: &str, chatroom_id: &str) -> Result<Value, Box<dyn Error>> {
    //     let params = json!({
    //         "appId": app_id,
    //         "chatroomId": chatroom_id,
    //     });
    //     util::gewe_post_json("/group/quitChatroom", Some(params)).await
    // }

    impl_params_api!(
    /// Disband chatroom/group
    /// TODO: need add doc
    disband_chatroom,
    "/group/disbandChatroom",
    ("appId", app_id, &str),
    ("chatroomId", chatroom_id, &str));
    // v0.1.0
    // pub async fn disband_chatroom(app_id: &str, chatroom_id: &str) -> Result<Value, Box<dyn Error>> {
    //     let params = json!({
    //         "appId": app_id,
    //         "chatroomId": chatroom_id,
    //     });
    //     util::gewe_post_json("/group/disbandChatroom", Some(params)).await
    // }

    impl_params_api!(
    /// Get chatroom/group info
    /// TODO: need add doc
    get_chatroom_info,
    "/group/getChatroomInfo",
    ("appId", app_id, &str),
    ("chatroomId", chatroom_id, &str));
    // v0.1.0
    // pub async fn get_chatroom_info(app_id: &str, chatroom_id: &str) -> Result<Value, Box<dyn Error>> {
    //     let params = json!({
    //         "appId": app_id,
    //         "chatroomId": chatroom_id,
    //     });
    //     util::gewe_post_json("/group/getChatroomInfo", Some(params)).await
    // }

    impl_params_api!(
    /// Get chatroom/group memberlist
    /// TODO: need add doc
    get_chatroom_member_list,
    "/group/getChatroomMemberList",
    ("appId", app_id, &str),
    ("chatroomId", chatroom_id, &str));
    // v0.1.0
    // pub async fn get_chatroom_member_list(
    //     app_id: &str,
    //     chatroom_id: &str,
    // ) -> Result<Value, Box<dyn Error>> {
    //     let params = json!({
    //         "appId": app_id,
    //         "chatroomId": chatroom_id,
    //     });
    //     util::gewe_post_json("/group/getChatroomMemberList", Some(params)).await
    // }

    impl_params_api!(
    /// Get chatroom/group detail
    /// TODO: need add doc
    get_chatroom_member_detail,
    "/group/getChatroomMemberDetail",
    ("appId", app_id, &str),
    ("chatroomId", chatroom_id, &str),
    ("memberWxids", member_wxids, Vec<Wxid>));
    // v0.1.0
    // pub async fn get_chatroom_member_detail(
    //     app_id: &str,
    //     chatroom_id: &str,
    //     member_wxids: Vec<Wxid>,
    // ) -> Result<Value, Box<dyn Error>> {
    //     let params = json!({
    //         "appId": app_id,
    //         "chatroomId": chatroom_id,
    //         "memberWxids": member_wxids,
    //     });
    //     util::gewe_post_json("/group/getChatroomMemberDetail", Some(params)).await
    // }

    impl_params_api!(
    /// Get chatroom/group announcement
    /// TODO: need add doc
    get_chatroom_announcement,
    "/group/getChatroomAnnouncement",
    ("appId", app_id, &str),
    ("chatroomId", chatroom_id, &str));
    // v0.1.0
    // pub async fn get_chatroom_announcement(
    //     app_id: &str,
    //     chatroom_id: &str,
    // ) -> Result<Value, Box<dyn Error>> {
    //     let params = json!({
    //         "appId": app_id,
    //         "chatroomId": chatroom_id,
    //     });
    //     util::gewe_post_json("/group/getChatroomAnnouncement", Some(params)).await
    // }

    impl_params_api!(
    /// Set chatroom/group announcement
    /// TODO: need add doc
    set_chatroom_announcement,
    "/group/setChatroomAnnouncement",
    ("appId", app_id, &str),
    ("chatroomId", chatroom_id, &str),
    ("content", content, &str));
    // v0.1.0
    // pub async fn set_chatroom_announcement(
    //     app_id: &str,
    //     chatroom_id: &str,
    //     content: &str,
    // ) -> Result<Value, Box<dyn Error>> {
    //     let params = json!({
    //         "appId": app_id,
    //         "chatroomId": chatroom_id,
    //         "content": content,
    //     });
    //     util::gewe_post_json("/group/setChatroomAnnouncement", Some(params)).await
    // }

    impl_params_api!(
    /// Agree join the chatroom (through invitation?)
    /// TODO: need add doc
    /// TODO: Set callback url to solve application
    agree_join_chatroom,
    "/group/agreeJoinRoom",
    ("appId", app_id, &str),
    ("chatroomName", url, &str));
    // v0.1.0
    // pub async fn agree_join_chatroom(app_id: &str, url: &str) -> Result<Value, Box<dyn Error>> {
    //     let params = json!({
    //         "appId": app_id,
    //         "chatroomName": url,
    //     });
    //     util::gewe_post_json("/group/agreeJoinRoom", Some(params)).await
    // }

    impl_params_api!(
    /// Add group member as friend
    /// TODO: need add doc
    add_group_member_as_friend,
    "/group/addGroupMemberAsFriend",
    ("appId", app_id, &str),
    ("memberWxid", member_wxid, &str),
    ("chatroomId", chatroom_id, &str),
    ("content", content, &str));
    // v0.1.0
    // pub async fn add_group_member_as_friend(
    //     app_id: &str,
    //     member_wxid: &str,
    //     chatroom_id: &str,
    //     content: &str,
    // ) -> Result<Value, Box<dyn Error>> {
    //     let params = json!({
    //         "appId": app_id,
    //         "memberWxid": member_wxid,
    //         "chatroomId": chatroom_id,
    //         "content": content,
    //     });
    //     util::gewe_post_json("/group/addGroupMemberAsFriend", Some(params)).await
    // }

    impl_params_api!(
    /// Get the chatroom qr_code
    /// TODO: need add doc
    get_chatroom_qr_code,
    "/group/getChatroomQrCode",
    ("appId", app_id, &str),
    ("chatroomId", chatroom_id, &str));
    // v0.1.0
    // pub async fn get_chatroom_qr_code(
    //     app_id: &str,
    //     chatroom_id: &str,
    // ) -> Result<Value, Box<dyn Error>> {
    //     let params = json!({
    //         "appId": app_id,
    //         "chatroomId": chatroom_id,
    //     });
    //     util::gewe_post_json("/group/getChatroomQrCode", Some(params)).await
    // }

    impl_params_api!(
    /// Save or remove chatroom/group to/from contract list
    /// TODO: need add doc
    /// TODO: add `operType` enum
    save_contract_list,
    "/group/saveContractList",
    ("appId", app_id, &str),
    ("operType", oper_type, u32),
    ("chatroomId", chatroom_id, &str));
    // v0.1.0
    // pub async fn save_contract_list(
    //     app_id: &str,
    //     oper_type: i32,
    //     chatroom_id: &str,
    // ) -> Result<Value, Box<dyn Error>> {
    //     let params = json!({
    //         "appId": app_id,
    //         "chatroomName": oper_type,
    //         "chatroomId": chatroom_id,
    //     });
    //     util::gewe_post_json("/group/saveContractList", Some(params)).await
    // }

    impl_params_api!(
    /// Admin operate
    /// TODO: need add doc
    /// TODO: add `operType` enum
    admin_operate,
    "/group/adminOperate",
    ("appId", app_id, &str),
    ("chatroomId", chatroom_id, &str),
    ("wxids", wxids, Vec<String>),
    ("operType", oper_type, u32));
    // v0.1.0
    // pub async fn admin_operate(
    //     app_id: &str,
    //     chatroom_id: &str,
    //     wxids: Vec<String>,
    //     oper_type: i32,
    // ) -> Result<Value, Box<dyn Error>> {
    //     let params = json!({
    //         "appId": app_id,
    //         "chatroomId": chatroom_id,
    //         "wxids": wxids,
    //         "operType": oper_type,
    //     });
    //     util::gewe_post_json("/group/adminOperate", Some(params)).await
    // }

    impl_params_api!(
    /// Pin chat or not
    /// TODO: need add doc
    pin_chat,
    "/group/pinChat",
    ("appId", app_id, &str),
    ("top", if_top, bool),
    ("chatroomId", chatroom_id, &str));
    // v0.1.0
    // pub async fn pin_chat(app_id: &str, top: bool, chatroom_id: &str) -> Result<Value, Box<dyn Error>> {
    //     let params = json!({
    //         "appId": app_id,
    //         "top": top,
    //         "chatroomId": chatroom_id,
    //     });
    //     util::gewe_post_json("/group/pinChat", Some(params)).await
    // }

    impl_params_api!(
    /// Set chatroom/group mute/silence mode
    /// TODO: need add doc
    set_group_silence,
    "/group/setMsgSilence",
    ("appId", app_id, &str),
    ("silence", if_silence, bool),
    ("chatroomId", chatroom_id, &str));
    // v0.1.0
    // pub async fn set_msg_silence(
    //     app_id: &str,
    //     silence: bool,
    //     chatroom_id: &str,
    // ) -> Result<Value, Box<dyn Error>> {
    //     let params = json!({
    //         "appId": app_id,
    //         "silence": silence,
    //         "chatroomId": chatroom_id,
    //     });
    //     util::gewe_post_json("/group/setMsgSilence", Some(params)).await
    // }

    impl_params_api!(
    /// Join chatroom/group using QR code
    /// Note: qr_url is parsing from the QR code image
    /// TODO: need add doc
    join_room_using_qr_code,
    "/group/joinRoomUsingQRCode",
    ("appId", app_id, &str),
    ("qrUrl", qr_url, &str));
    // v0.1.0
    // pub async fn join_room_using_qr_code(app_id: &str, qr_url: &str) -> Result<Value, Box<dyn Error>> {
    //     let params = json!({
    //         "appId": app_id,
    //         "qrUrl": qr_url,
    //     });
    //     util::gewe_post_json("/group/joinRoomUsingQRCode", Some(params)).await
    // }

    impl_params_api!(
    /// Check and approve chatroom/group access application
    /// Notice: need group invitation confirmaion enabled
    /// the group owner and administrators receive join requests
    /// TODO: need add doc
    check_room_application,
    "/group/roomAccessApplyCheckApprove",
    ("appId", app_id, &str),
    ("newMsgId", new_msg_id, &str),
    ("chatroomId", chatroom_id, &str),
    ("msgContent", msg_content, &str));
    // v0.1.0
    // pub async fn check_room_application(
    //     app_id: &str,
    //     new_msg_id: &str,
    //     chatroom_id: &str,
    //     msg_content: &str,
    // ) -> Result<Value, Box<dyn Error>> {
    //     let params = json!({
    //         "appId": app_id,
    //         "newMsgId": new_msg_id,
    //         "msgContent": msg_content,
    //         "chatroomId": chatroom_id,
    //     });
    //     util::gewe_post_json("/group/roomAccessApplyCheckApprove", Some(params)).await
    // }
}
