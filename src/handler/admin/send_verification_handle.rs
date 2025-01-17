use crate::config::app_res::{web_fail, web_success, web_success_data};
use crate::middleware::user_context::UserContext;
use crate::models::req::send_verification::SendVerificationReq;
use crate::service::admin::send_verification_service::send_verification_email;
use actix_web::{web, HttpMessage, HttpRequest, Responder};
use crate::models::req::captcha_req::CaptchaReqDTO;
use crate::models::res::captcha_res::CaptchaResDTO;
use crate::service::admin::send_verification_service;

#[utoipa::path(
    post,
    context_path = "/api/v1",
    path = "/sendVerification",
    request_body = SendVerificationReq,
    responses(
        (status = 200, description = "推送验证码成功")
    ),
    tag = "验证码"
)]
pub async fn send_verification(req: web::Json<SendVerificationReq>, r: HttpRequest) -> impl Responder {
    let reply = send_verification_email(req.into_inner(), r.extensions().get::<UserContext>().unwrap()).await;
    match reply {
        Ok(()) => web_success(),
        Err(_) => web_fail("发送失败，请稍后再试"),
    }
}


#[utoipa::path(
    get,
    context_path = "/api/v1",
    path = "/captcha",
    responses(
        (status = 200, description = "获取图形验证码验证码成功")
    ),
    tag = "验证码"
)]
pub async fn captcha() -> impl Responder {
    let reply = send_verification_service::captcha().await;
    match reply {
        Ok(res) => web_success_data::<CaptchaResDTO>(res),
        Err(_) => web_fail("获取失败，请稍后再试"),
    }
}

#[utoipa::path(
    get,
    context_path = "/api/v1",
    path = "/checkCaptcha",
    responses(
        (status = 200, description = "校验图形验证码验证码成功")
    ),
    tag = "验证码"
)]
pub async fn check_captcha(req: web::Json<CaptchaReqDTO>) -> impl Responder {
    let reply = send_verification_service::check_captcha(req).await;
    match reply {
        Ok(()) => web_success(),
        Err(_) => web_fail("校验失败，请稍后再试"),
    }
}