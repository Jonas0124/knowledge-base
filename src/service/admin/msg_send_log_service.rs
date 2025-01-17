use std::error::Error;
use chrono::{NaiveDateTime, Utc};
use diesel::dsl::insert_into;
use diesel::RunQueryDsl;
use crate::dao::init::db_connection;
use crate::middleware::user_context::UserContext;
use crate::models::entity::send_msg_log::MsgSendLog;
use crate::schema::send_msg_log::dsl::*;

pub fn  save_send_log(context: &UserContext, mt: i32,e: String,
                      s: i32, c: String, r: Option<String>
)  -> Result<(), Box<dyn Error>> {
    let msl = MsgSendLog {
        id: uuid::Uuid::new_v4().to_string(),
        user_id: context.id.clone(),
        msg_type: mt,
        email: e,
        success: s,
        verification_code: "".to_string(),
        verification_code_expire: 0,
        content: c,
        result: r,
        is_delete: "0".to_string(),
        create_time: Utc::now().naive_utc(),
        create_by: context.id.clone(),
        update_time: Utc::now().naive_utc(),
        update_by: context.id.clone(),
        reversion: 0,
    };
    let mut connection = db_connection().get().unwrap();
    insert_into(send_msg_log)
        .values(&msl)
        .execute(&mut connection)?;
    Ok(())
}