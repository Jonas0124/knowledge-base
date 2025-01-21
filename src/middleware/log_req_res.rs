// use actix_web::{dev::{ServiceRequest, ServiceResponse}, Error};
// use futures::future::{Ready, ready};
// use tracing::info;
//
// pub async fn log_request_and_response<B>(
//     req: ServiceRequest,
//     mut payload: ServiceResponse<B>,
// ) -> Ready<Result<ServiceResponse<B>, Error>> {
//     // 这里你可以处理请求和响应日志
//     // 比如打印请求的 query 字符串
//
//     info!("Response status: {}", req.query_string());
//     // 在这里你可能想要检查响应状态等
//     let fut = async {
//         let res = payload; // 这里是同步获取 ServiceResponse
//         info!("Response status: {}", res.status());
//         Ok(res)
//     };
//
//     // 使用 ready 返回同步的结果
//     ready(fut.await)
// }