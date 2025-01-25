use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct EmailVo<'a> {

    /// 邮件主题
    pub subject: &'a str,

    /// 邮件内容
    pub body: &'a str,

    /// 邮件内容类型:0-文字，1-html
    pub body_type: i32,

    /// 收件人，逗号分割
    pub to_address: &'a str,

    /// 消息类型
    pub msg_type: i32,
}