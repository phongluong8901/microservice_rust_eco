// ==========================================
// 1. IMPORT CÁC THƯ VIỆN CẦN THIẾT
// ==========================================

// Import các module xử lý HTTP, Router, Extension (chia sẻ state/pool) và các Extractor (như Path) từ Axum
use axum::{
    Extension, Json, Router, extract::Path, http::StatusCode, response::Response, routing::get,
};

// Import cấu hình ứng dụng (AppConfig) từ module configs2 và hàm kết nối database từ module dbs của dự án
#[allow(unused_imports)]
use proj_1::{AppError, AppResultJson, configs2::AppConfig, dbs::initialed_db};

// Import con trỏ thông minh Arc để chia sẻ dữ liệu an toàn giữa các luồng bất đồng bộ (async threads)
use std::sync::Arc;

// Import TcpListener của Tokio để mở cổng lắng nghe kết nối mạng TCP
use tokio::net::TcpListener;

// Import macro 'info' từ thư viện tracing để ghi log hệ thống
use serde_json::json;
use tracing::info;
// ==========================================
// 2. ĐỊNH NGHĨA CÁC HTTP HANDLER (HÀM XỬ LÝ REQUEST)
// ==========================================

/// Hàm `say_hello` nhận tham số `msg` từ URL (ví dụ: `/phong`) thông qua Axum Path Extractor.
/// Trả về `Result<String, String>`: Thành công trả về chuỗi chào, thất bại trả về thông báo lỗi.
// pub async fn say_hello(Path(msg): Path<String>) -> Result<String, String> {
//     if msg.is_empty() {
//         Err("Message cannot be empty".to_string())
//     } else {
//         Ok(format!("Hello from Axum, param: {}", msg))
//     }
// }

pub async fn say_hello(Path(msg): Path<String>) -> AppResultJson<serde_json::Value> {
    if msg.is_empty() {
        Err(AppError::NotFound)
    } else {
        Ok(Json(json!({"msg": "succ", "param": msg})))
    }
}

// ==========================================
// 3. ĐIỂM KHỞI CHẠY CHÍNH CỦA ỨNG DỤNG (MAIN)
// ==========================================

// Macro đánh dấu đây là hàm main chạy trên môi trường bất đồng bộ Tokio Runtime
#[tokio::main]
async fn main() {
    // Bước 1: Nạp các biến môi trường từ file .env vào chương trình
    dotenvy::dotenv().ok();

    // Bước 2: Khởi tạo hệ thống ghi log (tracing_subscriber)
    tracing_subscriber::fmt::init();
    info!("Starting service initialization...");

    // Bước 3: Đọc cấu hình tập trung từ biến môi trường thông qua AppConfig (configs2.rs)
    let cfg = AppConfig::from_env().expect("Cannot load config from environment");

    // Bước 4: Khởi động kết nối cơ sở dữ liệu PostgreSQL và chạy Migration tự động
    let pool = initialed_db(&cfg.postgres.dsn, cfg.postgres.max_conns).await;
    info!("Connect Database successfully!");

    // Bước 5: Khởi tạo Axum Router, đăng ký các đường dẫn (routes) và gắn Database Pool vào Extension
    let app = Router::new()
        // Định nghĩa route động dạng /{msg} (Sử dụng cú pháp chuẩn của Axum 0.7/0.8 trở lên)
        .route("/{msg}", get(say_hello))
        // Định nghĩa route kiểm tra nhanh database
        .route("/db_check", get(|| async { "Check DB" }))
        // Bọc pool vào Arc và đưa vào Extension để các handler bên trên có thể gọi lại khi cần query DB
        .layer(Extension(Arc::new(pool)));

    // Bước 6: Lấy cấu hình địa chỉ cổng (addr) từ AppConfig
    let addr = cfg.web.addr;
    info!("Binding server to address: {}", addr);

    // Bước 7: Mở cổng TCP lắng nghe. Nếu cổng bị chiếm hoặc lỗi, sẽ in thông báo cảnh báo rõ ràng
    let listener = TcpListener::bind(&addr).await.unwrap_or_else(|err| {
        panic!(
            "Khong the mo cong {}: {}. Hay doi sang cong khac trong file .env!",
            addr, err
        );
    });

    info!("Server is running and listening on http://{}", addr);

    // Bước 8: Khởi chạy HTTP Server của Axum bằng TcpListener và Router đã cấu hình ở trên
    axum::serve(listener, app).await.unwrap();
}
