

pub enum RedisEnum {
    CreateUserEmailSend,
    UpdateUserEmailSend,
    LogOffUser,
    CAPTCHA,
    LogInUser,
}

impl RedisEnum {
    pub fn to_key(&self) -> &'static str {
        match self {
            RedisEnum::CreateUserEmailSend => "user:create:",
            RedisEnum::UpdateUserEmailSend => "user:update:",
            RedisEnum::LogOffUser => "user:logOff:",
            RedisEnum::CAPTCHA => "captcha:create:",
            RedisEnum::LogInUser => "user:login:",
        }
    }

    pub fn get_key(cap_type: &i32) -> RedisEnum {
        if cap_type == &0 {
            return RedisEnum::CreateUserEmailSend;
        }else if cap_type == &1 {
            return RedisEnum::UpdateUserEmailSend;
        }else if cap_type == &2 {
            return RedisEnum::LogOffUser;
        }
        RedisEnum::CAPTCHA
    }
}