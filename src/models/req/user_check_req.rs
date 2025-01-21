use getset::{Getters, Setters};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use utoipa::IntoParams;

#[derive(Serialize, Deserialize, ToSchema, IntoParams, Debug, Getters, Setters)]
#[into_params(parameter_in = Query)]
pub struct UserCheckReqDTO {
    /// 用户名
    #[getset(get = "pub", set = "pub")]
    username: String,
    /// 邮箱
    #[getset(get = "pub", set = "pub")]
    email: String,
}
