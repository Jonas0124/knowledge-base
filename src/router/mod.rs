use actix_web::{App, HttpServer};
use actix_web::web;
use utoipa::{Modify, OpenApi};
use utoipa::openapi::security::{ApiKey, ApiKeyValue, SecurityScheme};
use utoipa_swagger_ui::SwaggerUi;
use crate::handler::{admin, user};
use crate::handler::ping::ping;
use crate::middleware::auth::AuthMiddleware;
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
        crate::handler::admin::user::create,
        crate::handler::admin::user::update_password,
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
                .wrap(UserContextMiddleware)
                .service(web::resource("/login").route(web::post().to(user::login)))
                .service(web::resource("/sendVerification").route(web::post().to(admin::send_verification_handle::send_verification)))
                .service(
                    web::scope("/admin")
                        .wrap(AuthMiddleware)
                        .service(web::resource("/user/create").route(web::post().to(admin::user::create)))
                        .service(web::resource("/user/updatePassword").route(web::post().to(admin::user::update_password)))
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
        App::new().configure(config_app)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
