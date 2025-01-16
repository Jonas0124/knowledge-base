use crate::config::app_res::{web_fail, web_success};
use crate::middleware::user_context::UserContext;
use crate::models::req::send_verification::SendVerificationReq;
use crate::service::admin::send_verification_service::send_verification_email;
use actix_web::{web, HttpMessage, HttpRequest, Responder};

#[utoipa::path(
    post,
    context_path = "/api/v1",
    path = "/sendVerification",
    request_body = SendVerificationReq,
    responses(
        (status = 200, description = "推送验证码成功")
    ),
    tag = "用户模块"
)]
pub async fn send_verification(req: web::Json<SendVerificationReq>, r: HttpRequest) -> impl Responder {
    let reply = send_verification_email(req.into_inner(), r.extensions().get::<UserContext>().unwrap()).await;
    match reply {
        Ok(token) => web_success(),
        Err(err) => web_fail("发送失败，请稍后再试"),
    }
}