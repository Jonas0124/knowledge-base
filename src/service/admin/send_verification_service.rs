use std::error::Error;
use r2d2_redis::redis::Commands;
use rand::Rng;
use crate::dao::redis_db::get_redis_connection;
use crate::middleware::user_context::UserContext;
use crate::models::req::send_verification::SendVerificationReq;
use crate::models::vo::email_vo::EmailVo;
use crate::service::open::open_service::send_email;

pub async fn send_verification_email(req: SendVerificationReq, context: &UserContext) -> Result<(), Box<dyn Error>> {
    // 推送验证码
    let mut rng = rand::thread_rng();  // 获取随机数生成器
    let random_number = rng.gen_range(100_000..1_000_000).to_string();  // 生成 6 位数字
    let email_vo = EmailVo {
        subject: "创建用户验证码",
        body_type: 1,
        to_address: &req.email,
        body: &"<a href=\"https://www.baidu.com\">点击这里跳转</a>",
        // body: &("<h1>创建用户验证码如下,过期时间10分钟:</h1>\n<h2>".to_string() + &random_number + "</h2>"),
    };
    send_email(&email_vo, context).await?;
    let mut connection = get_redis_connection().await.unwrap();
    connection.set_ex::<&str, i32, i32>("yanzheng", 789897, 600)?;
    Ok(())
}