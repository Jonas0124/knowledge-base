use crate::config::app_res::business_err;
use crate::dao::init::db_connection;
use crate::dao::redis_db::get_redis_connection;
use crate::dao::user_basic_dao::User;
use crate::define::JWT_SECRET;
use crate::handler::user::{UserClaim, UserLoginRequest};
use crate::models::r#enum::redis_enum::RedisEnum;
use crate::schema::user::dsl::*;
use diesel::prelude::*;
use diesel::{BoolExpressionMethods, ExpressionMethods, RunQueryDsl};
use jsonwebtoken::{encode, EncodingKey, Header};
use r2d2_redis::redis::Commands;
use std::error::Error;
use std::io::ErrorKind;

pub async fn login_service(req: UserLoginRequest) -> Result<String, Box<dyn Error>> {
    let mut connection = db_connection().get().unwrap();

    // 2. 获取 user 信息
    let res = user.filter(email.eq(&req.username).or(username.eq(&req.username)))
        .first::<User>(&mut connection)
        .ok();
    let Some(res_user) = res else {
        return business_err(ErrorKind::NotFound, "用户不存在");
    };
    if req.password != res_user.password {
        return business_err(ErrorKind::NotFound, "密码错误");
    }

    // 4. 生成 token
    let claim = UserClaim {
        id: res_user.id,
        username: res_user.username,
        email: res_user.email,
        exp: 60 * 60,
    };
    let token = encode(
        &Header::default(), &claim, &EncodingKey::from_secret(JWT_SECRET.as_ref())
    )?;
    let mut conn = get_redis_connection().await?;
    conn.set_ex(RedisEnum::LogInUser.to_key().to_string() + &token, claim.email, claim.exp)?;
    Ok(token)
}