use getset::{Getters, Setters};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema, Getters, Setters)]
pub struct UserCheckReqDTO {
    /// 用户名
    #[getset(get = "pub", set = "pub")]
    username: String,
    /// 邮箱
    #[getset(get = "pub", set = "pub")]
    email: String,
}
