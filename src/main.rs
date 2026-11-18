// ==========================================
// 1. IMPORT CÁC THƯ VIỆN CẦN THIẾT
// ==========================================

use axum::{
    Extension,
    Json,
    Router,
    extract::Path,
    http::{Method, StatusCode, Uri},
    middleware,
    response::{IntoResponse, Response}, // Đã thêm IntoResponse
    routing::get,
};

#[allow(unused_imports)]
use proj_1::{AppError, AppResultJson, configs2::AppConfig, dbs::initialed_db};

use serde_json::json;
use std::sync::Arc;
use tokio::net::TcpListener;
use tracing::info;
use uuid::Uuid; // Đã thêm import Uuid chuẩn

// ==========================================
// 2. ĐỊNH NGHĨA CÁC HTTP HANDLER (HÀM XỬ LÝ REQUEST)
// ==========================================

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

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();
    info!("Starting service initialization...");

    let cfg = AppConfig::from_env().expect("Cannot load config from environment");
    let pool = initialed_db(&cfg.postgres.dsn, cfg.postgres.max_conns).await;
    info!("Connect Database successfully!");

    let app = Router::new()
        .route("/{msg}", get(say_hello))
        .route("/db_check", get(|| async { "Check DB" }))
        .layer(middleware::map_response(mx_map_response))
        .layer(Extension(Arc::new(pool)));

    let addr = cfg.web.addr;
    info!("Binding server to address: {}", addr);

    let listener = TcpListener::bind(&addr).await.unwrap_or_else(|err| {
        panic!(
            "Khong the mo cong {}: {}. Hay doi sang cong khac trong file .env!",
            addr, err
        );
    });

    info!("Server is running and listening on http://{}", addr);

    axum::serve(listener, app).await.unwrap();
}

// ==========================================
// 4. MIDDLEWARE MAP_RESPONSE (ĐÃ SỬA CHUẨN THỨ TỰ)
// ==========================================

// Lưu ý: Trong Axum, tham số `res: Response` PHẢI đứng trước các extractor như `Uri`, `Method`
pub async fn mx_map_response(uri: Uri, method: Method, res: Response) -> Response {
    let request_id = Uuid::new_v4(); // Sửa lại đúng chính tả Uuid
    info!(
        "Request ID: {} | Method: {} | URI: {}",
        request_id, method, uri
    );

    // Bọc lại response kèm StatusCode và gọi .into_response()
    (StatusCode::OK, res).into_response()
}
