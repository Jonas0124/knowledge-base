use actix_cors::Cors;
use actix_web::{App, HttpServer};
use actix_web::web;
use utoipa::{Modify, OpenApi};
use utoipa::openapi::security::{ApiKey, ApiKeyValue, SecurityScheme};
use utoipa_swagger_ui::SwaggerUi;
use crate::handler::{admin, user};
use crate::handler::ping::ping;
use crate::middleware::auth::AuthMiddleware;
use crate::middleware::log_middlware::LogMiddleware;
use crate::middleware::user_context::UserContextMiddleware;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "KnowledgeBase",
        version = "24.1.0",
        description = "个人知识库"
    ),
    paths(
        crate::handler::ping::ping,
        crate::handler::user::login,
        crate::handler::admin::send_verification_handle::send_verification,
        crate::handler::admin::send_verification_handle::captcha,
        crate::handler::admin::send_verification_handle::check_captcha,
        crate::handler::admin::user::create,
        crate::handler::admin::user::check_user,
        crate::handler::admin::user::update_password,
        crate::handler::admin::user::log_off,
        crate::handler::admin::user::log_out,
        crate::handler::admin::user::list,
    ),
    components(schemas(
        crate::handler::user::UserLoginRequest,
        crate::handler::admin::user::UserCreateRequest,
        crate::handler::admin::user::UserResetPasswordRequest,
        crate::handler::admin::user::UserSecretReqDTO,
        crate::handler::admin::user::UserListRequest,
        crate::handler::admin::user::UserListReply,
        crate::models::req::send_verification::SendVerificationReq,
        crate::models::req::captcha_req::CaptchaReqDTO,
        crate::models::req::user_log_off_req::UserLogOffReqDTO,
        crate::models::req::user_check_req::UserCheckReqDTO,
        crate::models::res::captcha_res::CaptchaResDTO,
    )),
    modifiers(&SecurityAddon)
)]
struct ApiDoc;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "Authorization",
                SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("Authorization"))),
            )
        }
    }
}

fn config_app(cfg: &mut web::ServiceConfig) {
    cfg
        .service(web::resource("/ping").route(web::get().to(ping)))
        .service(
            web::scope("/api/v1")
                .wrap(LogMiddleware)
                .wrap(UserContextMiddleware)
                .service(web::resource("/captcha").route(web::get().to(admin::send_verification_handle::captcha)))//获取图形验证码
                .service(web::resource("/checkCaptcha").route(web::post().to(admin::send_verification_handle::check_captcha)))//校验图形验证码
                .service(web::resource("/login").route(web::post().to(user::login)))//登陆
                .service(web::resource("/sendVerification").route(web::post().to(admin::send_verification_handle::send_verification)))//发送邮箱验证码
                .service(web::resource("/user/checkUser").route(web::get().to(admin::user::check_user)))//用户唯一校验
                .service(web::resource("/user/create").route(web::post().to(admin::user::create)))//创建用户
                .service(
                    web::scope("admin")
                        .wrap(AuthMiddleware)
                        .service(web::resource("/user/updatePassword").route(web::post().to(admin::user::update_password)))
                        .service(web::resource("/user/logOff").route(web::post().to(admin::user::log_off)))
                        .service(web::resource("/user/logOut/{id}").route(web::get().to(admin::user::log_out)))
                        .service(web::resource("/user/list").route(web::get().to(admin::user::list)))
                )
        )
        .service(
            SwaggerUi::new("/swagger-ui/{_:.*}")
                .url("/api-docs/openapi.json", ApiDoc::openapi()),
        );
}

pub async fn run_server() ->std::io::Result<()> {

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin() // 允许所有来源
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
            .allowed_headers(vec![
                http::header::AUTHORIZATION,
                http::header::ACCEPT,
                http::header::CONTENT_TYPE,

            ])
            .supports_credentials()
            .max_age(3600);
        App::new().wrap(cors).configure(config_app)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
