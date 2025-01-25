use crate::service::user::login_service;
use actix_web::{web, HttpResponse, Responder};
use getset::{Getters, Setters};
use serde::{Deserialize, Serialize};
use serde_json::json;
use utoipa::ToSchema;
use validator_derive::Validate;

#[derive(Serialize, Deserialize, ToSchema, Validate, Getters, Setters)]
pub struct UserLoginRequest {
    /// 用户名或油箱
    #[validate(length(min = 1, message = "用户名不能为空"))]
    #[getset(get = "pub", set = "pub")]
    username: String,
    /// 密码
    #[validate(length(min = 1, message = "密码不能为空"))]
    #[getset(get = "pub", set = "pub")]
    password: String,
}

#[derive(Serialize, Deserialize)]
pub struct UserClaim {
    // 唯一标识
    pub id: String,
    // 用户名
    pub username: String,
    // 邮箱地址
    pub email: String,
    // 过期时间
    pub exp: i64,
}

#[utoipa::path(
    post,
    context_path = "/api/v1",
    path = "/login",
    request_body = UserLoginRequest,
    responses(
        (status = 200, description = "登录成功")
    ),
    tag = "用户模块"
)]
pub async fn login(req: actix_web_validator::Json<UserLoginRequest>) -> impl Responder {
    let reply = login_service(req.into_inner()).await;
    match reply {
        Ok(token) => HttpResponse::Ok().json(json!({"code": 200, "data": {"token": token}})),
        Err(err) => HttpResponse::Ok().json(json!({"code": -1, "msg": err.to_string()}))
    }
}