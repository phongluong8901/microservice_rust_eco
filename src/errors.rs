use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
}; //Import các thành phần của Axum để xử lý HTTP: Json (trả về dữ liệu dạng JSON), StatusCode (các mã lỗi HTTP chuẩn như 400, 404), IntoResponse và Response (trait và kiểu dữ liệu cho phép một struct/enum tự động biến đổi thành HTTP Response).
use serde::Serialize; //Import trait Serialize từ serde để cho phép các struct (như WebResponse) được chuyển đổi thành chuỗi JSON.
use serde_json::{Value, json}; //Import kiểu dữ liệu tổng quát Value (đại diện cho bất kỳ kiểu dữ liệu JSON nào) và macro json! (giúp viết cấu trúc JSON trực quan như trong JavaScript).
use thiserror::Error; //Import macro Error từ thư viện thiserror (giúp viết các custom error trong Rust cực kỳ gọn gàng mà không cần tự implement trait std::error::Error thủ công).

#[derive(Debug, Error)] //Gắn macro để struct/enum tự động có khả năng in log (Debug) và biến thành một Error chuẩn của Rust (thiserror::Error).
pub enum AppError {
    //#[from]: Đây là tính năng cực kỳ mạnh mẽ của thiserror. Nó tự động tạo ra hàm chuyển đổi (From) từ lỗi config::ConfigError sang AppError. Nhờ vậy, khi bạn dùng dấu chấm hỏi ? ở các hàm đọc config, lỗi sẽ tự động được ép kiểu sang AppError::Config mà không cần gọi .map_err(...) thủ công.
    #[error("Configuration Error: {0}")]
    Config(#[from] config::ConfigError),

    #[error("Not found...")]
    NotFound,
}

//#[derive(Debug, Serialize)]: Cho phép struct này tự động chuyển đổi thành JSON để trả về cho client.
#[derive(Debug, Serialize)]
//pub struct WebResponse<T Value>: Định nghĩa một struct generic với kiểu dữ liệu T. Nếu không truyền kiểu cụ thể, mặc định T sẽ là serde_json::Value. Giúp bạn tái sử dụng cho mọi loại dữ liệu trả về (User, List, Token, v.v.).
pub struct WebResponse<T = Value> {
    pub data: T,
    pub code: u16,
    pub status: bool,
}

//impl IntoResponse for AppError: Triển khai trait IntoResponse của Axum cho AppError. Nhờ đoạn này, bất cứ khi nào hàm handler của bạn trả về Result<T, AppError>, Axum sẽ tự động biết cách chuyển đổi AppError thành một phản hồi HTTP hoàn chỉnh mà không cần xử lý thủ công ở từng route.
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        //match self: Dùng biểu thức match để phân rã từng trường hợp lỗi trong enum AppError và trả về mã HTTP kèm body JSON tương ứng.
        match self {
            AppError::Config(e) => (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "status": false,
                    "code": StatusCode::BAD_REQUEST.as_u16(),
                    "error": e.to_string() //Cuối cùng gọi .into_response() để biến tuple chứa StatusCode và Json thành kiểu dữ liệu Response chung của Axum.
                })),
            )
                .into_response(),
            AppError::NotFound => (
                StatusCode::NOT_FOUND,
                Json(json!({
                    "status": false,
                    "code": StatusCode::NOT_FOUND.as_u16(),
                    "error": "Not found"
                })),
            )
                .into_response(),
        }
    }
}
