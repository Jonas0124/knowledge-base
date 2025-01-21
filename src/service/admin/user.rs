use std::collections::HashMap;
use crate::config::app_res::business_err;
use crate::dao::init::db_connection;
use crate::dao::user_basic_dao::User;
use crate::dao::user_secret::UserSecret;
use crate::handler::admin::user::{UserCreateRequest, UserListRequest, UserResetPasswordRequest};
use crate::schema::user::dsl as user_dsl;
use crate::schema::user_secret::dsl::{user_secret, id};
use diesel::dsl::insert_into;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::{BoolExpressionMethods, ExpressionMethods, MysqlConnection, QueryDsl, RunQueryDsl};
use std::error::Error;
use std::io::ErrorKind;
use r2d2_redis::redis::Commands;
use uuid::Uuid;
use crate::dao::redis_db::get_redis_connection;
use crate::middleware::user_context::UserContext;
use crate::models::r#enum::redis_enum::RedisEnum;
use crate::models::req::user_check_req::UserCheckReqDTO;

pub async fn create_service(req: UserCreateRequest, context: &UserContext) -> Result<(), Box<dyn Error>> {
    // 验证码校验
    let mut conn = get_redis_connection().await?;
    let option = conn.get::<&str, String>(&(RedisEnum::CreateUserEmailSend.to_key().to_string() + &req.email)).ok();
    let Some(option) = option else {
        return business_err(ErrorKind::Other, "验证码错误");
    };
    if option != req.verification_content {
        return business_err(ErrorKind::Other, "验证码错误");
    }
    // 1. db client
    let pool = db_connection();
    let mut conn: PooledConnection<ConnectionManager<MysqlConnection>> = pool.get()?;
    // 2. username 存在
    check_user(&req, &mut conn)?;
    let user_id = Uuid::new_v4().to_string();
    let user_db = User {
        id: user_id.clone(),
        username: req.username,
        password: req.password,
        email: req.email,
        is_delete: String::from("0"),
        reversion: 0,
        create_time: chrono::Utc::now().naive_utc(),
        update_time: chrono::Utc::now().naive_utc(),
        create_by: context.id.to_string(),
        update_by: context.id.to_string(),
    };

    insert_into(user_dsl::user)
        .values(&user_db)
        .execute(&mut conn)?;

    Ok(())
}

/// 用户唯一校验
pub fn check_user(req: &UserCreateRequest, mut conn: &mut PooledConnection<ConnectionManager<MysqlConnection>>) -> Result<(), Box<dyn Error>> {
    let count: i64 = user_dsl::user.filter(user_dsl::username.eq(&req.username).or(user_dsl::email.eq(&req.email)))
        .count().get_result(&mut conn)?;

    if count > 0 {
        return business_err(ErrorKind::AlreadyExists, "用户名或者邮箱地址已存在");
    }
    Ok(())
}


pub async fn reset_password_service(req: UserResetPasswordRequest) -> Result<(), Box<dyn Error>> {
    let mut connection = db_connection().get().unwrap();
    let user_option = user_dsl::user.find(&req.id).get_result::<User>(&mut connection).ok();
     let Some(user_res) = user_option else {
         return business_err(ErrorKind::NotFound, "用户不存在");
     };
    let question_ids: Vec<String> = req.user_secret_req.iter().map(|x| x.id.clone()).collect();
    let questios_option = user_secret.filter(id.eq_any(&question_ids)).load::<UserSecret>(&mut connection).ok();
    let Some(questions) = questios_option else {
        return business_err(ErrorKind::Other, "验证失败");
    };
    if questions.len() != 3 {
        return business_err(ErrorKind::Other, "验证失败");
    }
    let da_secret_map: HashMap<&String, &UserSecret> = questions.iter().map(|question| (&question.id, question)).collect();

    for item in req.user_secret_req.iter() {
        let option = da_secret_map.get(&item.id);
        let Some(secret) = option else {
            return business_err(ErrorKind::Other, "验证失败");
        };
        if item.answer != secret.answer {
            return business_err(ErrorKind::Other, "验证失败");
        }
    }


    //验证成功，修改密码
    let result = diesel::update(user_dsl::user)
        .filter(user_dsl::id.eq(&req.id).and(user_dsl::reversion.eq(&user_res.reversion)))
        .set(user_dsl::password.eq(req.password))
        .execute(&mut connection).ok();
    let Some(num) = result else {
        return business_err(ErrorKind::Other, "业务繁忙，请重试！");
    };
    if num < 1 {
        return business_err(ErrorKind::Other, "业务繁忙，请重试！");
    }
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

pub(crate) fn check_user_uk(req: UserCheckReqDTO) -> Result<(), Box<dyn Error>> {
    let pool = db_connection();
    let mut conn: PooledConnection<ConnectionManager<MysqlConnection>> = pool.get()?;
    let request = UserCreateRequest {
        username: req.username().to_string(),
        email: req.email().to_string(),
        .. Default::default()
    };
    check_user(&request, &mut conn)?;
    Ok(())
}