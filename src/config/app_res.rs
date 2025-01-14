use actix_web::HttpResponse;
use serde_json::json;
use std::error::Error;
use std::io;
use std::io::ErrorKind;

pub fn web_success() -> HttpResponse {
    HttpResponse::Ok().json(json!({"code": 200, "data": "ok"}))
}

pub fn web_success_data<T>(data: T) -> HttpResponse
where T: serde::Serialize
{
    HttpResponse::Ok().json(json!({"code": 200, "data": data}))
}
pub fn web_fail(msg: &str) -> HttpResponse {
    HttpResponse::Ok().json(json!({"code": -1, "data": msg}))
}

pub fn business_err<T>(error_kind: ErrorKind, msg: &str) -> Result<T, Box<dyn Error>>{

    Err(Box::new(io::Error::new(error_kind, msg)))
}



