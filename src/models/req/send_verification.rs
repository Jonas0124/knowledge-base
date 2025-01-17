use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct SendVerificationReq {


    /// 邮箱
    pub email: String,

    /// 消息类型：0-创建账户，1-修改密码，2-注销账户
    pub msg_type: i32,


}