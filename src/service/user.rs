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
use chrono::{Duration, Utc};
use crate::util::argon2util::verify_password;

pub async fn login_service(req: UserLoginRequest) -> Result<String, Box<dyn Error>> {
    let mut connection = db_connection().get().unwrap();

    // 2. 获取 user 信息
    let res = user.filter(email.eq(&req.username()).or(username.eq(&req.username())))
        .first::<User>(&mut connection)
        .ok();
    let Some(res_user) = res else {
        return business_err(ErrorKind::NotFound, "用户不存在");
    };
    if res_user.is_delete.ne("0") {
        return business_err(ErrorKind::NotFound, "用户不存在");
    }
    if !verify_password(req.password(), &res_user.password) {
        return business_err(ErrorKind::NotFound, "密码错误");
    }

    // 4. 生成 token
    let claim = UserClaim {
        id: res_user.id,
        username: res_user.username,
        email: res_user.email,
        exp: Utc::now().timestamp_millis() + Duration::hours(1).num_milliseconds(),
    };
    let token = encode(
        &Header::default(), &claim, &EncodingKey::from_secret(JWT_SECRET.as_ref())
    )?;
    let mut conn = get_redis_connection().await?;
    conn.set_ex(RedisEnum::LogInUser.to_key().to_string() + &token, claim.email, 60 * 60)?;
    Ok(token)
}