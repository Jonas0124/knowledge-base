use crate::config::app_res::{web_fail, web_success, web_success_data};
use crate::dao::user_basic_dao::User;
use crate::middleware::user_context::UserContext;
use crate::service::admin::user::{create_service, list_service, log_off_service, log_out_service, reset_password_service};
use actix_web::{web, HttpMessage, HttpRequest, Responder};
use serde::{Deserialize, Serialize};
use serde_json::json;
use utoipa::{IntoParams, ToSchema};
use crate::config::log_init::req_log;
use crate::models::req::user_check_req::UserCheckReqDTO;
use crate::models::req::user_log_off_req::UserLogOffReqDTO;
use crate::service::admin::user;

#[derive(Serialize, Deserialize, ToSchema, Default)]
pub struct UserCreateRequest {
    /// 用户名
    pub username: String,
    /// 密码
    pub password: String,
    /// 邮箱
    pub email: String,

    /// 验证码校验
    pub verification_content: String,

}

// #[derive(Serialize, Deserialize, ToSchema)]
// pub struct CreateUserSecretReqDTO {
//
//     /// 问题
//     pub question: String,
//
//     /// 答案
//     pub answer: String,
// }


#[utoipa::path(
    post,
    context_path = "/api/v1",
    path = "/user/create",
    request_body = UserCreateRequest,
    responses(
        (status = 200, description = "创建成功")
    ),
    tag = "超管模块-用户管理",
    security(("Authorization" = []))
)]
pub async fn create(req: web::Json<UserCreateRequest>, r: HttpRequest) -> impl Responder {
    let reply = create_service(req.into_inner(), r.extensions().get::<UserContext>().unwrap()).await;
    match reply {
        Ok(_) => web_success(),
        Err(err) => web_fail(&err.to_string())
    }
}


#[derive(Serialize, Deserialize, ToSchema)]
pub struct UserResetPasswordRequest {
    /// 唯一标识
    pub id: String,

    /// 密码
    pub password: String,

    /// 邮箱
    pub email: String,

    /// 验证码校验
    pub verification_content: String,


}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct UserSecretReqDTO {
    /// 问题id
    pub id: String,

    /// 答案
    pub answer: String,
}

#[utoipa::path(
    post,
    context_path = "/api/v1",
    path = "/admin/user/updatePassword",
    request_body = UserResetPasswordRequest,
    responses(
        (status = 200, description = "修改密码")
    ),
    tag = "超管模块-用户管理",
    security(("Authorization" = []))
)]
pub async fn update_password(req: web::Json<UserResetPasswordRequest>) -> impl Responder {
    let reply = reset_password_service(req.into_inner()).await;
    match reply {
        Ok(_) => web_success(),
        Err(err) => web_fail(&err.to_string())
    }
}

#[utoipa::path(
    post,
    context_path = "/api/v1",
    path = "/admin/user/logOff",
    request_body = UserLogOffReqDTO,
    responses(
        (status = 200, description = "注销账户")
    ),
    tag = "超管模块-用户管理",
    security(("Authorization" = []))
)]
pub async fn log_off(req: web::Json<UserLogOffReqDTO>, r: HttpRequest) -> impl Responder {
    let token = r.headers().get("Authorization");
    let Some(token) = token else {
        return web_fail("token失效");
    };
    let Some(token) = token.to_str().ok() else {
        return web_fail("token失效");
    };
    let reply = log_off_service(req.into_inner(), token).await;
    match reply {
        Ok(_) => web_success(),
        Err(err) => web_fail(&err.to_string())
    }
}

#[utoipa::path(
    get,
    context_path = "/api/v1",
    path = "/admin/user/logOut/{id}",
    responses(
        (status = 200, description = "退出登录")
    ),
    tag = "超管模块-用户管理",
    security(("Authorization" = []))
)]
pub async fn log_out(req: web::Path<i32>, r: HttpRequest) -> impl Responder {
    let token = r.headers().get("Authorization");
    let Some(token) = token else {
        return web_success();
    };
    let Some(token) = token.to_str().ok() else {
        return web_success();
    };
    let re = req.into_inner();
    req_log(&re);
    let reply = log_out_service(token).await;
    match reply {
        Ok(_) => web_success(),
        Err(err) => web_fail(&err.to_string())
    }
}

#[derive(Serialize, Deserialize, ToSchema, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct UserListRequest {
    /// 当前页
    pub page: i64,
    /// 每页的数据条数
    pub size: i64,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct UserListReply {
    /// 数据
    pub list: Vec<User>,
    /// 总数
    pub total: i64,
}

#[utoipa::path(
    get,
    context_path = "/api/v1",
    path = "/admin/user/list",
    params(UserListRequest),
    responses(
        (status = 200, description = "用户列表", body = UserListReply)
    ),
    tag = "超管模块-用户管理",
    security(("Authorization" = []))
)]
pub async fn list(req: web::Query<UserListRequest>) -> impl Responder {
    let reply = list_service(req.into_inner()).await;
    match reply {
        Ok(result) => web_success_data(result),
        Err(err) => web_fail(&err.to_string())
    }
}

#[utoipa::path(
    get,
    context_path = "/api/v1",
    path = "/user/checkUser",
    params(UserCheckReqDTO),
    responses(
        (status = 200, description = "用户唯一校验")
    ),
    tag = "超管模块-用户管理",
    security(("Authorization" = []))
)]
pub async fn check_user(req: web::Query<UserCheckReqDTO>) -> impl Responder {
    let dto = req.into_inner();
    req_log(&dto);
    let reply = user::check_user_uk(dto);
    match reply {
        Ok(()) => web_success(),
        Err(err) => web_fail(&err.to_string())
    }
}
