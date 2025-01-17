use chrono::{DateTime, NaiveDateTime, Utc};
use diesel::{AsChangeset, Insertable, Queryable};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema, Insertable, Queryable)]
#[diesel(table_name = crate::schema::send_msg_log)]
pub struct MsgSendLog {

    /// 消息id
    pub id: String,

    /// 用户id
    pub user_id: String,

    /// 消息类型：0-创建账户，1-修改密码，2-注销账户
    pub msg_type: i32,

    /// 邮箱
    pub email: String,

    /// 推送是否成(0-失败，1成功)
    pub success: i32,

    /// 验证码
    pub verification_code: String,

    /// 验证码过期时间戳
    pub verification_code_expire: i64,

    /// 推送内容
    pub content: String,

    /// 推送结果
    pub result: Option<String>,

    /// 是否删除
    pub is_delete: String,

    /// 创建时间
    pub create_time: NaiveDateTime,

    /// 创建人
    pub create_by: String,

    /// 更新时间
    pub update_time: NaiveDateTime,

    /// 修改人
    pub update_by: String,

    /// 版本号
    pub reversion: i32,
}