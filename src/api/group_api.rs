use crate::{user::Wxid, util};
use serde_json::{json, Value};
use std::error::Error;

/// Ctreate chatroom/group with at least two users
pub async fn create_chatroom(app_id: &str, wxids: Vec<Wxid>) -> Result<Value, Box<dyn Error>> {
    let params = json!({
        "appId": app_id,
        "wxids": wxids,
    });
    util::gewe_post_json("/group/createChatroom", Some(params)).await
}

/// Modify chatroom/group name
pub async fn modify_chatroom_name(
    app_id: &str,
    chatroom_name: &str,
    chatroom_id: &str,
) -> Result<Value, Box<dyn Error>> {
    let params = json!({
        "appId": app_id,
        "chatroomName": chatroom_name,
        "chatroomId": chatroom_id,
    });
    util::gewe_post_json("/group/modifyChatroomName", Some(params)).await
}

/// Modify chatroom/group remark (local)
pub async fn modify_chatroom_remark(
    app_id: &str,
    chatroom_remark: &str,
    chatroom_id: &str,
) -> Result<Value, Box<dyn Error>> {
    let params = json!({
        "appId": app_id,
        "chatroomRemark": chatroom_remark,
        "chatroomId": chatroom_id,
    });
    util::gewe_post_json("/group/modifyChatroomRemark", Some(params)).await
}

/// Modify chatroom/group nickname
pub async fn modify_chatroom_nickname(
    app_id: &str,
    nick_name: &str,
    chatroom_id: &str,
) -> Result<Value, Box<dyn Error>> {
    let params = json!({
        "appId": app_id,
        "nickName": nick_name,
        "chatroomId": chatroom_id,
    });
    util::gewe_post_json("/group/modifyChatroomNickNameForSelf", Some(params)).await
}

/// Invite new chatroom/group members
pub async fn invite_member(
    app_id: &str,
    wxids: Vec<Wxid>,
    chatroom_id: &str,
    reason: &str,
) -> Result<Value, Box<dyn Error>> {
    let params = json!({
        "appId": app_id,
        "wxids": wxids,
        "chatroomId": chatroom_id,
        "reason": reason,
    });
    util::gewe_post_json("/group/inviteMember", Some(params)).await
}

/// Remove members
pub async fn remove_member(
    app_id: &str,
    wxids: Vec<Wxid>,
    chatroom_id: &str,
) -> Result<Value, Box<dyn Error>> {
    // TODO double check
    let flatten_wxids = wxids
        .iter()
        .map(|wxid| wxid.to_string())
        .collect::<Vec<String>>()
        .join(",");

    let params = json!({
        "appId": app_id,
        "wxids": flatten_wxids,
        "chatroomId": chatroom_id,
    });
    util::gewe_post_json("/group/removeMember", Some(params)).await
}

/// Quit chatroom/group
pub async fn quit_chatroom(app_id: &str, chatroom_id: &str) -> Result<Value, Box<dyn Error>> {
    let params = json!({
        "appId": app_id,
        "chatroomId": chatroom_id,
    });
    util::gewe_post_json("/group/quitChatroom", Some(params)).await
}

/// Disband chatroom/group
pub async fn disband_chatroom(app_id: &str, chatroom_id: &str) -> Result<Value, Box<dyn Error>> {
    let params = json!({
        "appId": app_id,
        "chatroomId": chatroom_id,
    });
    util::gewe_post_json("/group/disbandChatroom", Some(params)).await
}

/// Get chatroom/group info
pub async fn get_chatroom_info(app_id: &str, chatroom_id: &str) -> Result<Value, Box<dyn Error>> {
    let params = json!({
        "appId": app_id,
        "chatroomId": chatroom_id,
    });
    util::gewe_post_json("/group/getChatroomInfo", Some(params)).await
}

/// Get chatroom/group memberlist
pub async fn get_chatroom_member_list(
    app_id: &str,
    chatroom_id: &str,
) -> Result<Value, Box<dyn Error>> {
    let params = json!({
        "appId": app_id,
        "chatroomId": chatroom_id,
    });
    util::gewe_post_json("/group/getChatroomMemberList", Some(params)).await
}

/// Get chatroom/group detail
pub async fn get_chatroom_member_detail(
    app_id: &str,
    chatroom_id: &str,
    member_wxids: Vec<Wxid>,
) -> Result<Value, Box<dyn Error>> {
    let params = json!({
        "appId": app_id,
        "chatroomId": chatroom_id,
        "memberWxids": member_wxids,
    });
    util::gewe_post_json("/group/getChatroomMemberDetail", Some(params)).await
}

/// Get chatroom/group announcement
pub async fn get_chatroom_announcement(
    app_id: &str,
    chatroom_id: &str,
) -> Result<Value, Box<dyn Error>> {
    let params = json!({
        "appId": app_id,
        "chatroomId": chatroom_id,
    });
    util::gewe_post_json("/group/getChatroomAnnouncement", Some(params)).await
}

/// Set chatroom/group announcement
pub async fn set_chatroom_announcement(
    app_id: &str,
    chatroom_id: &str,
    content: &str,
) -> Result<Value, Box<dyn Error>> {
    let params = json!({
        "appId": app_id,
        "chatroomId": chatroom_id,
        "content": content,
    });
    util::gewe_post_json("/group/setChatroomAnnouncement", Some(params)).await
}

/// Agree join the chatroom
/// TODO: Set callback url to solve application
pub async fn agree_join_chatroom(app_id: &str, url: &str) -> Result<Value, Box<dyn Error>> {
    let params = json!({
        "appId": app_id,
        "chatroomName": url,
    });
    util::gewe_post_json("/group/agreeJoinRoom", Some(params)).await
}

/// 添加群成员为好友
pub async fn add_group_member_as_friend(
    app_id: &str,
    member_wxid: &str,
    chatroom_id: &str,
    content: &str,
) -> Result<Value, Box<dyn Error>> {
    let params = json!({
        "appId": app_id,
        "memberWxid": member_wxid,
        "chatroomId": chatroom_id,
        "content": content,
    });
    util::gewe_post_json("/group/addGroupMemberAsFriend", Some(params)).await
}

/// 获取群二维码
pub async fn get_chatroom_qr_code(
    app_id: &str,
    chatroom_id: &str,
) -> Result<Value, Box<dyn Error>> {
    let params = json!({
        "appId": app_id,
        "chatroomId": chatroom_id,
    });
    util::gewe_post_json("/group/getChatroomQrCode", Some(params)).await
}

/// 群保存到通讯录
pub async fn save_contract_list(
    app_id: &str,
    oper_type: i32,
    chatroom_id: &str,
) -> Result<Value, Box<dyn Error>> {
    let params = json!({
        "appId": app_id,
        "chatroomName": oper_type,
        "chatroomId": chatroom_id,
    });
    util::gewe_post_json("/group/saveContractList", Some(params)).await
}

/// 管理员操作
pub async fn admin_operate(
    app_id: &str,
    chatroom_id: &str,
    wxids: Vec<String>,
    oper_type: i32,
) -> Result<Value, Box<dyn Error>> {
    let params = json!({
        "appId": app_id,
        "chatroomId": chatroom_id,
        "wxids": wxids,
        "operType": oper_type,
    });
    util::gewe_post_json("/group/adminOperate", Some(params)).await
}

/// 聊天置顶
pub async fn pin_chat(app_id: &str, top: bool, chatroom_id: &str) -> Result<Value, Box<dyn Error>> {
    let params = json!({
        "appId": app_id,
        "top": top,
        "chatroomId": chatroom_id,
    });
    util::gewe_post_json("/group/pinChat", Some(params)).await
}

/// 设置消息免打扰
pub async fn set_msg_silence(
    app_id: &str,
    silence: bool,
    chatroom_id: &str,
) -> Result<Value, Box<dyn Error>> {
    let params = json!({
        "appId": app_id,
        "silence": silence,
        "chatroomId": chatroom_id,
    });
    util::gewe_post_json("/group/setMsgSilence", Some(params)).await
}

/// 扫码进群
pub async fn join_room_using_qr_code(app_id: &str, qr_url: &str) -> Result<Value, Box<dyn Error>> {
    let params = json!({
        "appId": app_id,
        "qrUrl": qr_url,
    });
    util::gewe_post_json("/group/joinRoomUsingQRCode", Some(params)).await
}

/// 确认进群申请
pub async fn room_access_apply_check_approve(
    app_id: &str,
    new_msg_id: &str,
    chatroom_id: &str,
    msg_content: &str,
) -> Result<Value, Box<dyn Error>> {
    let params = json!({
        "appId": app_id,
        "newMsgId": new_msg_id,
        "msgContent": msg_content,
        "chatroomId": chatroom_id,
    });
    util::gewe_post_json("/group/roomAccessApplyCheckApprove", Some(params)).await
}
