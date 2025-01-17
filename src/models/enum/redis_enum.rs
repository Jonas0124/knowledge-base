

pub enum RedisEnum {
    CreateUserEmailSend,
    CAPTCHA,
}

impl RedisEnum {
    pub fn to_key(&self) -> &'static str {
        match self {
            RedisEnum::CreateUserEmailSend => "user:create:",
            RedisEnum::CAPTCHA => "captcha:create:",
        }
    }
}