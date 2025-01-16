// @generated automatically by Diesel CLI.

diesel::table! {
    send_msg_log (id) {
        #[max_length = 64]
        id -> Varchar,
        #[max_length = 64]
        user_id -> Varchar,
        msg_type -> Integer,
        #[max_length = 64]
        email -> Varchar,
        success -> Integer,
        #[max_length = 10]
        verification_code -> Varchar,
        verification_code_expire -> Bigint,
        #[max_length = 3000]
        content -> Varchar,
        result -> Nullable<Text>,
        #[max_length = 64]
        is_delete -> Varchar,
        create_time -> Datetime,
        #[max_length = 64]
        create_by -> Varchar,
        update_time -> Datetime,
        #[max_length = 64]
        update_by -> Varchar,
        reversion -> Integer,
    }
}

diesel::table! {
    user (id) {
        #[max_length = 64]
        id -> Varchar,
        #[max_length = 64]
        username -> Varchar,
        #[max_length = 255]
        password -> Varchar,
        #[max_length = 64]
        email -> Varchar,
        #[max_length = 64]
        is_delete -> Varchar,
        create_time -> Datetime,
        #[max_length = 64]
        create_by -> Varchar,
        update_time -> Datetime,
        #[max_length = 64]
        update_by -> Varchar,
        reversion -> Integer,
    }
}

diesel::table! {
    user_secret (id) {
        #[max_length = 64]
        id -> Varchar,
        #[max_length = 64]
        user_id -> Varchar,
        #[max_length = 255]
        question -> Varchar,
        #[max_length = 255]
        answer -> Varchar,
        #[max_length = 64]
        is_delete -> Varchar,
        create_time -> Datetime,
        #[max_length = 64]
        create_by -> Varchar,
        update_time -> Datetime,
        #[max_length = 64]
        update_by -> Varchar,
        reversion -> Integer,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    send_msg_log,
    user,
    user_secret,
);
