use crate::service::user::login_service;
use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use serde_json::json;
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct UserLoginRequest {
    /// 用户名或油箱
    pub username: String,
    /// 密码
    pub password: String,
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
    pub exp: usize,
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
pub async fn login(req: web::Json<UserLoginRequest>) -> impl Responder {
    let reply = login_service(req.into_inner()).await;
    match reply {
        Ok(token) => HttpResponse::Ok().json(json!({"code": 200, "data": {"token": token}})),
        Err(err) => HttpResponse::Ok().json(json!({"code": -1, "msg": err.to_string()}))
    }
}