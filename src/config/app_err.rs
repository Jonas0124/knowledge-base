use std::error::Error;
use std::io;
use std::io::ErrorKind;
use serde_json::json;
use crate::config::app_err;


use serde_json::Value;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct JsonError(Value);

impl fmt::Display for JsonError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "JSON Error: {}", self.0)
    }
}

impl Error for JsonError {}


pub fn web_err(code: i32, msg: &str) -> Result<(), actix_web::error::Error> {
    Err(actix_web::error::ErrorUnauthorized(json!({"code": code, "msg": msg})))
}

pub fn business_err(app_err: AppErr) -> Result<(), Box<dyn Error>>{
    let e = app_err.get_err();
    // Err(Box::new(json!({"code": e.0, "msg": e.1})))
    JsonError(json!({"error": e}))
}

pub enum AppErr<'a> {
    CUSTOM(i32, &'a str),


}
impl AppErr<'static> {

    pub fn get_err(&self) -> (i32, &str) {
        match self {
            AppErr::CUSTOM(code, msg) => (*code, msg),
        }
    }
    pub const USER_NOT_FIND: AppErr<'static> = AppErr::CUSTOM(5000, "用户名不存在");
}
