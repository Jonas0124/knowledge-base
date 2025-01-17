use std::error::Error;
use std::io::Cursor;
use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use captcha::Captcha;
use captcha::filters::{Noise, Wave};
use image::{DynamicImage, ImageFormat, RgbaImage};
use image::imageops::{brighten, contrast};
use r2d2_redis::redis::Commands;
use rand::Rng;
use crate::dao::redis_db::get_redis_connection;
use crate::middleware::user_context::UserContext;
use crate::models::req::send_verification::SendVerificationReq;
use crate::models::res::captcha_res::CaptchaResDTO;
use crate::models::vo::email_vo::EmailVo;
use crate::service::open::open_service::send_email;


/// 发邮件
pub async fn send_verification_email(req: SendVerificationReq, context: &UserContext) -> Result<(), Box<dyn Error>> {
    // 推送验证码
    let mut rng = rand::thread_rng();  // 获取随机数生成器
    let random_number = rng.gen_range(100_000..1_000_000).to_string();  // 生成 6 位数字
    let email_vo = EmailVo {
        subject: "创建用户验证码",
        body_type: 1,
        to_address: &req.email,
        // body: &"<a href=\"https://www.baidu.com\">点击这里跳转</a>",
        body: &("<h1>创建用户验证码如下,过期时间10分钟:</h1>\n<h2>".to_string() + &random_number + "</h2>"),
        msg_type: req.msg_type,
    };
    send_email(&email_vo, context).await?;
    let mut connection = get_redis_connection().await.unwrap();
    connection.set_ex::<&str, i32, i32>("yanzheng", 789897, 600)?;
    Ok(())
}

/// 图形验证码
pub fn captcha() -> Result<CaptchaResDTO, Box<dyn Error>> {
    let mut captcha = Captcha::new();

    // 添加字符、波浪效果、噪点
    captcha
        .add_chars(5)                   // 生成5位验证码字符
        .apply_filter(Wave::new(2.0, 8.0))   // 添加波浪效果
        .apply_filter(Noise::new(0.4))      // 添加噪点
        .view(200, 100);                  // 设置验证码图像的大小

    // 获取图像的 PNG 数据
    let captcha_image_data = captcha.as_png().unwrap();

    // 将图像数据转换为 RgbaImage
    let captcha_image = image::load_from_memory(&captcha_image_data)
        .unwrap()
        .to_rgba8();

    // 进行适度模糊
    let blurred_image = apply_moderate_blur(captcha_image);

    // 增强图像的亮度和对比度
    let enhanced_image = enhance_image(blurred_image);

    // 获取验证码字符
    let captcha_text: String = captcha.chars().iter().collect();

    // 将处理后的图像编码为 Base64
    let mut enhanced_image_data = Vec::new();
    let mut cursor = Cursor::new(&mut enhanced_image_data);
    enhanced_image.write_to(&mut cursor, ImageFormat::Png).unwrap();  // 使用 Cursor 包装 Vec

    // 使用 base64::engine::general_purpose::STANDARD 编码
    let captcha_image_base64 = STANDARD.encode(&enhanced_image_data);

    // 生成唯一 ID
    let captcha_id: String = rand::thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(16)
        .map(char::from)
        .collect();

    // 将验证码存储到redis并返回

    Ok(CaptchaResDTO {
        captcha_id: captcha_id,
        captcha_image: captcha_image_base64
    })

}




// 适当模糊图像函数（减少模糊强度）
fn apply_moderate_blur(image: RgbaImage) -> RgbaImage {
    let mut img = DynamicImage::ImageRgba8(image);
    img = img.blur(0.8);  // 降低模糊强度
    img.to_rgba8()
}

// 处理图像的亮度和对比度
fn enhance_image(image: RgbaImage) -> RgbaImage {
    let mut img = DynamicImage::ImageRgba8(image);

    // 增加亮度
    brighten(&mut img, 20);  // 增加亮度

    // 增加对比度
    contrast(&mut img, 50.0); // 增加对比度

    img.to_rgba8()
}