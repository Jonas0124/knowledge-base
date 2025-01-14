use chrono::NaiveDateTime;
use diesel::{AsChangeset, Insertable, Queryable};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema, Insertable, Queryable, AsChangeset)]
#[diesel(table_name = crate::schema::user_secret)]
pub struct UserSecret {

    /// 问题id
    pub id: String,

    /// 用户id
    pub user_id: String,

    /// 问题
    pub question: String,

    /// 答案
    pub answer: String,

    /// 是否删除
    pub is_delete: String,

    /// 创建时间
    pub create_time: NaiveDateTime,

    /// 创建人
    pub create_by: String,


    /// 更新时间
    pub update_time: NaiveDateTime,


    /// 修改人
    pub update_by: String,

    /// 版本号
    pub reversion: i32,
}