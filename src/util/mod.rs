pub mod argon2util;












// use std::error::Error;
// use reqwest::Client;
// use serde_json::{json, Value};
// use tokio::sync::Mutex;
// use futures_util::stream::StreamExt;
// use std::sync::Arc;
//
// #[tokio::main]
// async fn main() {
//     let chat_history = Arc::new(Mutex::new(Vec::<Value>::new()));
//
//     let user_input = "我叫wp";
//
//     let response = handle_request(user_input, chat_history.clone()).await.unwrap();
//     println!("{}", response);
//
//     // 第二次对话（带有上下文）
//     let user_input2 = "如果你返回了一大堆换行符或者其它东西，我如何展示到页面？";
//     let response2 = handle_request(user_input2, chat_history.clone()).await.unwrap();
//     println!("{}", response2);
// }
//
// async fn handle_request(user_input: &str, chat_history: Arc<Mutex<Vec<Value>>>) -> Result<String, Box<dyn Error>> {
//     let client = Client::new();
//     let url = "https://spark-api-open.xf-yun.com/v1/chat/completions";
//
//     let mut history = chat_history.lock().await;
//
//     // 添加用户的新输入
//     history.push(json!({
//         "role": "user",
//         "content": user_input
//     }));
//
//     // 发送请求
//     let response = client
//         .post(url)
//         .header("Authorization", "Bearer UvqVdfXdRNWdvSIRLvtU:fslFIeEJKAGCcdzGfZuq") // 替换为你的 API 密钥
//         .json(&json!({
//             "model": "lite",
//             "messages": *history,  // 传递完整的历史记录
//             "stream": false
//         }))
//         .send()
//         .await?;
//
//     let mut response_text = String::new();
//     let mut stream = response.bytes_stream();
//
//     while let Some(chunk) = stream.next().await {
//         match chunk {
//             Ok(data) => {
//                 let chunk_str = String::from_utf8_lossy(&data);
//                 response_text.push_str(&chunk_str);
//             }
//             Err(_) => break,
//         }
//     }
//
//     // 记录 AI 的回答
//     history.push(json!({
//         "role": "assistant",
//         "content": response_text
//     }));
//
//     // **可选：限制历史长度**
//     // if history.len() > 10 {
//     //     history.drain(0..history.len() - 10);
//     // }
//
//     Ok(response_text)
// }
