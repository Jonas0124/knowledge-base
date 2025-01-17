use crate::middleware::user_context::UserContext;
use crate::models::res::email_res::EmailRes;
use crate::models::vo::email_vo::EmailVo;
use crate::service::admin::msg_send_log_service::save_send_log;
use base64::Engine;
use chrono::Utc;
use dotenvy::dotenv;
use reqwest::Client;
use std::collections::HashMap;
use std::env;
use std::error::Error;
use base64::engine::general_purpose::STANDARD;
use url::{form_urlencoded, Url};
use uuid::Uuid;

/// 邮箱推送




pub async fn send_email(email_vo: &EmailVo<'_>, context: &UserContext) -> Result<(), Box<dyn Error>> {
    dotenv().ok(); // 加载 .env 文件中的环境变量

    let app_id = env::var("ACCESS_KEY_ID")
        .expect("ACCESS_KEY_ID must be set in .env file");
    let app_key = env::var("ACCESS_KEY_SECRET")
        .expect("ACCESS_KEY_SECRET must be set in .env file");
    // 生成签名
    let uid = Uuid::new_v4().to_string();
    let time = Utc::now().to_rfc3339();
    // 构造最终的 URL
    let mut url = Url::parse("https://dm.aliyuncs.com/")?;
    {
        let mut params1 = url.query_pairs_mut();
        let mut params = HashMap::new();
        params.insert("AccessKeyId", app_id.as_str());
        params.insert("Action", "SingleSendMail");
        params.insert("AccountName", "woja@email.woja.top");//发送人
        params.insert("ReplyToAddress", "true");
        params.insert("ReplyAddress", "weipeng618@qq.com");//
        params.insert("AddressType", "0");
        params.insert("ToAddress", email_vo.to_address);//收信人,逗号分割（100个上限）
        params.insert("Subject", email_vo.subject);
        if email_vo.body_type == 0 {
            params.insert("TextBody", email_vo.body);
        }else {
            params.insert("HtmlBody", email_vo.body);
        }
        params.insert("Format", "JSON");
        params.insert("Version", "2015-11-23");
        params.insert("SignatureMethod", "HMAC-SHA1");
        params.insert("SignatureVersion", "1.0");
        params.insert("SignatureNonce", uid.as_str());
        params.insert("Timestamp", time.as_str());
        params.insert("RegionId", "cn-hangzhou");

        // 按字典顺序排序参数
        let mut sorted_params: Vec<(&&str, &&str)> = params.iter().collect();
        sorted_params.sort_by(|a, b| a.0.cmp(b.0));  // 按照键名进行字典排序

        // 构造规范化的请求字符串
        let canonicalized_query_string: String = sorted_params.iter()
            .map(|(k, v)| {
                params1.append_pair(k, v);
                format!("{}={}", percent_encode(k), percent_encode(v))
            })
            .collect::<Vec<String>>()
            .join("&");

        // 构造待签名字符串
        let string_to_sign = format!("GET&%2F&{}", percent_encode(&canonicalized_query_string));

        // 计算签名
        let sha1 = hmac_sha1::hmac_sha1(format!("{}&", app_key).as_bytes(), string_to_sign.as_bytes());
        let signature = STANDARD.encode(sha1);

        // 将签名添加到参数中
        params1.append_pair("Signature", &signature);
    }

    // 发送请求
    let client = Client::new();
    let response = client.get(url.clone())
        .send()
        .await?
        .text()
        .await?;
    // 保存推送数据
    let option = serde_json::from_str::<EmailRes>(&response).ok();
    let mut  success = 1;
    if let Option::Some(option) = option {
        if let None = option.EnvId {
            success = 0;
        }
    }else {
        success = 0;
    }
    println!("{}", success);
    let em = email_vo.to_address;
    save_send_log(context, email_vo.msg_type, em.to_string(), success, url.to_string(), Some(response))?;
    Ok(())
}

fn percent_encode(s: &str) -> String {
    form_urlencoded::byte_serialize(s.as_bytes()).collect()
}
