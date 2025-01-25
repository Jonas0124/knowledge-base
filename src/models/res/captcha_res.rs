use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct CaptchaResDTO {

    /// 图形验证码唯一标识
    pub  captcha_id: String,

    /// 图形验证码内容
    pub captcha_image: String,


}