use crate::handler::user::{UserClaim, UserLoginRequest};
use crate::schema::user::dsl::*;
use diesel::{BoolExpressionMethods, ExpressionMethods, RunQueryDsl};
use std::error::Error;
use std::io::ErrorKind;
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use crate::dao::user_basic_dao::User;
use crate::define::JWT_SECRET;
use diesel::prelude::*;
use crate::config::app_res::{business_err};
use crate::dao::init::db_connection;

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
        exp: Utc::now().timestamp_millis() + Duration::days(1).num_milliseconds(),
    };
    let token = encode(
        &Header::default(), &claim, &EncodingKey::from_secret(JWT_SECRET.as_ref())
    )?;

    Ok(token)
}