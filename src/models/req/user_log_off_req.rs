use getset::{Getters, Setters};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Debug, Getters, Setters, ToSchema)]
pub struct UserLogOffReqDTO {

    /// 唯一标识
    #[getset(get = "pub", set = "pub")]
    id: String,

    /// 邮箱
    #[getset(get = "pub", set = "pub")]
    email: String,

    /// 验证码校验
    #[getset(get = "pub", set = "pub")]
    verification_content: String,

}