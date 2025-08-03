use crate::dao::init::db_connection;
use crate::middleware::user_context::UserContext;
use crate::models::entity::send_msg_log::MsgSendLog;
use crate::schema::send_msg_log::dsl::*;
use chrono::{Local, Utc};
use diesel::dsl::insert_into;
use diesel::{sql_query, RunQueryDsl};
use std::error::Error;
use std::io::ErrorKind;
use diesel::sql_types::Text;
use crate::config::app_res::{business_err, web_fail};

pub fn  save_send_log(context: &UserContext, mt: i32, e: String,
                      s: i32, c: String, r: Option<String>
) -> Result<(), Box<dyn Error>> {
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

pub fn check_count()  -> Result<(), Box<dyn Error>> {
    let today: chrono::NaiveDate = Local::now().date_naive();
    let mut connection = db_connection().get().unwrap();
    let count = sql_query(
        "
    SELECT COUNT(*) AS count
    FROM send_msg_log
    WHERE DATE(create_time) = ?
    GROUP BY create_time
    "
    )
        .bind::<Text, _>(today.to_string()).execute(&mut connection).unwrap();
    if count > 100 {
        return business_err::<()>(ErrorKind::Other, "今日验证码推送已达上线");
    }
    Ok(())
}