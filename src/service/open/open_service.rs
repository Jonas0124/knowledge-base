use crate::models::vo::email_vo::EmailVo;
use base64::encode;
use chrono::Utc;
use hmac_sha1::hmac_sha1;
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use reqwest::Client;
use std::collections::HashMap;
use std::error::Error;
use std::fmt::Debug;
use url::{form_urlencoded, Url};
use uuid::Uuid;

/// 邮箱推送

const ACCESS_KEY_ID: &str = "";
const ACCESS_KEY_SECRET: &str = "";



pub async fn sen_email(email_vo: &EmailVo<'_>) -> Result<(), Box<dyn Error>> {
    // 基础信息
    let action = "SingleSendMail"; // 替换为你的操作名称RFC3339

    let timestamp = Utc::now().to_rfc3339();
    let signature_nonce = uuid::Uuid::new_v4().to_string(); // 替换为实际的唯一随机值

    // 拼接 StringToSign
    let method = "GET";
    let uri = "/"; // 请求的 API 路径
    let query_params = format!(
        "AccessKeyId={}&Action={}&Format=xml&SignatureMethod=HMAC-SHA1&SignatureNonce={}&SignatureVersion=1.0&Timestamp={}&Version=2015-11-23",
        ACCESS_KEY_ID,
        action,
        signature_nonce,
        utf8_percent_encode(&timestamp, NON_ALPHANUMERIC)
    );
    let string_to_sign = format!("{}&{}&{}", method, utf8_percent_encode(uri, NON_ALPHANUMERIC), utf8_percent_encode(&query_params, NON_ALPHANUMERIC));

    // 生成签名
    let signature = base64::encode(hmac_sha1(ACCESS_KEY_SECRET.as_bytes(), string_to_sign.as_bytes()));
    let uid = Uuid::new_v4().to_string();
    let time = Utc::now().to_rfc3339();
    // 构造最终的 URL
    let mut url = Url::parse("https://dm.aliyuncs.com/")?;
    {
        let mut params1 = url.query_pairs_mut();
        let mut params = HashMap::new();
        params.insert("AccessKeyId", ACCESS_KEY_ID);
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
        let sha1 = hmac_sha1::hmac_sha1(format!("{}&", ACCESS_KEY_SECRET).as_bytes(), string_to_sign.as_bytes());
        let signature = encode(sha1);

        // 将签名添加到参数中
        params.insert("Signature", &signature);
        params1.append_pair("Signature", &signature);
    }

    // 发送请求
    let client = Client::new();
    let response = client.get(url)
        .send()
        .await?
        .text()
        .await?;

    Ok(())
}

pub fn percent_encode(s: &str) -> String {
    form_urlencoded::byte_serialize(s.as_bytes()).collect()
}