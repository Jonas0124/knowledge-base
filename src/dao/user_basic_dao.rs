use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema, Insertable, Queryable)]
#[diesel(table_name = crate::schema::user)]
pub struct User {
    /// 唯一标识
    pub id: String,
    /// 用户名
    pub username: String,
    /// 密码
    pub password: String,
    /// 邮箱
    pub email: String,

    /// 是否删除
    pub is_delete: String,

    /// 版本号
    pub reversion: i32,

    /// 创建人
    pub create_by: String,

    /// 创建时间
    pub create_time: NaiveDateTime,

    /// 修改人
    pub update_by: String,

    /// 更新时间
    pub update_time: NaiveDateTime,
}

pub const USER_BASIC_DAO: &str = "user_basic_dao";
