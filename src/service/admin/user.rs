use std::error::Error;
use std::io;
use std::io::ErrorKind;
use diesel::{BoolExpressionMethods, ExpressionMethods, MysqlConnection, QueryDsl, RunQueryDsl};
use diesel::dsl::insert_into;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::sql_types::BigInt;
use serde_json::{from_value, json, Value};
use uuid::Uuid;
use crate::config::app_res::business_err;
use crate::dao::init::db_connection;
use crate::dao::user_basic_dao::{USER_BASIC_DAO, User};
use crate::handler::admin::user::{UserCreateRequest, UserListRequest, UserListReply, UserResetPasswordRequest};
use crate::schema::user::dsl::*;

pub async fn create_service(req: UserCreateRequest) -> Result<(), Box<dyn Error>> {
    // 1. db client
    let pool = db_connection();
    let mut conn: PooledConnection<ConnectionManager<MysqlConnection>> = pool.get()?;
    // 2. username 存在
    let count: i64 = user.filter(username.eq(&req.username).or(email.eq(&req.email)))
        .count().get_result(&mut conn)?;
    // if count > 0 {
    //     return Err(Box::new(io::Error::new(ErrorKind::AlreadyExists,"用户名或者邮箱地址已存在")))
    // }
    if count > 0 {
        return business_err(ErrorKind::AlreadyExists, "用户名或者邮箱地址已存在");
    }
    let user_db = User {
        id: Uuid::new_v4().to_string(),
        username: req.username,
        password: req.password,
        email: req.email,
        is_delete: String::from("0"),
        reversion: 0,
        create_time: chrono::Utc::now().naive_utc(),
        update_time: chrono::Utc::now().naive_utc(),
        create_by: String::from("0"),
        update_by: String::from("0"),
    };

    insert_into(user)
        .values(&user_db)
        .execute(&mut conn)?;

    Ok(())
}

pub async fn reset_password_service(req: UserResetPasswordRequest) -> Result<(), Box<dyn Error>> {
    // 1. es client
    // let client = es_client();

    // 2. 获取 user 信息
    // let response = client.search(SearchParts::Index(&[USER_BASIC_DAO]))
    //     .body(json!({
    //         "query": {
    //             "term": {
    //                 "uuid.keyword": req.uuid
    //             }
    //         }
    //     }))
    //     .send()
    //     .await;
    // if let Err(e) = response {
    //     return Err(Box::new(e))
    // }
    // let response = response?;
    // let response_body = response.json::<Value>().await?;
    //
    // let user_id = response_body["hits"]["hits"][0]["_id"].as_str()
    //     .ok_or_else(|| io::Error::new(ErrorKind::NotFound, "获取用户信息失败"))?;
    //
    // // 3. save
    // client.update(UpdateParts::IndexId(USER_BASIC_DAO, user_id))
    //     .body(json!({
    //         "doc": {
    //             "password": req.password,
    //             "update_at": chrono::Utc::now().timestamp_millis()
    //         }
    //     }))
    //     .send()
    //     .await?;

    Ok(())
}

pub async fn list_service(req: UserListRequest) -> Result<(), Box<dyn Error>> {
    // 1. es client
    // let client = es_client();
    //
    // // 2. 准备查询条件
    // let response = client.search(SearchParts::Index(&[USER_BASIC_DAO]))
    //     .body(
    //         json!({
    //             "size": req.size,
    //             "from": (req.page - 1) * req.size,
    //             "sort": [
    //                 {
    //                     "create_at": {
    //                         "order": "desc"
    //                     }
    //                 }
    //             ],
    //             "query": {
    //                 "match_all": {}
    //             }
    //         })
    //     )
    //     .send()
    //     .await;
    // if let Err(e) = response {
    //     return Err(Box::new(e))
    // }
    // let response = response?;
    // let response_body = response.json::<Value>().await?;
    //
    // // total
    // let total = response_body["hits"]["total"]["value"].as_i64().unwrap();
    // // list
    // let hits = response_body["hits"]["hits"].as_array().unwrap();
    //
    // // 3. 结果处理
    // let list = hits.iter()
    //     .map(|hit| from_value(hit["_source"].clone()).unwrap())
    //     .collect::<Vec<UserBasicDao>>();

    Ok(())
}
