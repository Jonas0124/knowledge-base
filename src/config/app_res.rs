use actix_web::HttpResponse;
use serde_json::json;
use std::error::Error;
use std::io;
use std::io::ErrorKind;
use tracing::info;

pub fn web_success() -> HttpResponse {
    info!("成功统一响应:{}", "ok");
    HttpResponse::Ok().json(json!({"code": 200, "data": "ok"}))
}

pub fn web_success_data<T>(data: T) -> HttpResponse
where T: serde::Serialize
{
    info!("成功统一响应:{}", json!(data));
    HttpResponse::Ok().json(json!({"code": 200, "data": data}))
}
pub fn web_fail(msg: &str) -> HttpResponse {
    info!("失败统一响应:{}", msg);
    HttpResponse::Ok().json(json!({"code": -1, "data": msg}))
}

pub fn business_err<T>(error_kind: ErrorKind, msg: &str) -> Result<T, Box<dyn Error>>{
    info!("异常统一响应:{}", msg);
    Err(Box::new(io::Error::new(error_kind, msg)))
}



