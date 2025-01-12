// @generated automatically by Diesel CLI.

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
