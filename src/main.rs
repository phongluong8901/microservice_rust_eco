// ==========================================
// 1. IMPORT CÁC THƯ VIỆN CẦN THIẾT
// ==========================================

use axum::{
    Extension,
    Json,
    Router,
    extract::{Path, Request, State}, // Thêm Request và State cho Middleware
    http::{Method, StatusCode, Uri},
    middleware::{self, Next}, // Thêm Next cho Middleware
    response::{IntoResponse, Response},
    routing::get,
};

#[allow(unused_imports)]
use proj_1::{AppError, AppResultJson, configs2::AppConfig, dbs::initialed_db};

use serde_json::json;
use sqlx::PgPool; // Import PgPool để làm state cho mx_auth
use std::sync::Arc;
use tokio::net::TcpListener;
use tracing::info;
use uuid::Uuid;

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
    // Bước 1: Nạp biến môi trường và khởi tạo log
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();
    info!("Starting service initialization...");

    // Bước 2: Đọc cấu hình từ AppConfig
    let cfg = AppConfig::from_env().expect("Cannot load config from environment");

    // Bước 3: Kết nối Database và chạy migration
    let pool = initialed_db(&cfg.postgres.dsn, cfg.postgres.max_conns).await;
    info!("Connect Database successfully!");

    // Bước 4: Khởi tạo Router và gắn các middleware, extension
    let app = Router::new()
        .route("/{msg}", get(say_hello))
        .route("/db_check", get(|| async { "Check DB" }))
        // Gắn middleware map_response (thứ tự chạy sau cùng)
        .layer(middleware::map_response(mx_map_response))
        // Gắn middleware auth có truyền state là pool (thứ tự chạy trước handler)
        .layer(middleware::from_fn_with_state(pool.clone(), mx_auth))
        // Gắn Extension chứa Arc<PgPool> để Handler gọi lại khi cần query DB
        .layer(Extension(Arc::new(pool)));

    // Bước 5: Lấy cấu hình địa chỉ và mở cổng TCP Listener
    let addr = cfg.web.addr;
    info!("Binding server to address: {}", addr);

    let listener = TcpListener::bind(&addr).await.unwrap_or_else(|err| {
        panic!(
            "Khong the mo cong {}: {}. Hay doi sang cong khac trong file .env!",
            addr, err
        );
    });

    info!("Server is running and listening on http://{}", addr);

    // Bước 6: Khởi chạy Axum Server
    axum::serve(listener, app).await.unwrap();
}

// ==========================================
// 4. CÁC MIDDLEWARE
// ==========================================

/// Middleware ghi log thông tin Response (Request ID, Method, URI)
pub async fn mx_map_response(uri: Uri, method: Method, res: Response) -> Response {
    let request_id = Uuid::new_v4();
    info!(
        "Request ID: {} | Method: {} | URI: {}",
        request_id, method, uri
    );

    (StatusCode::OK, res).into_response()
}

/// Middleware xác thực sử dụng State (PgPool) và chuyển tiếp request qua Next
pub async fn mx_auth(
    State(pool): State<PgPool>, // Nhận state pool để dùng nếu cần kiểm tra DB
    req: Request,               // Nhận request hiện tại
    next: Next,                 // Nhận next để cho phép request đi tiếp
) -> Result<Response, AppError> {
    info!("MIDDLEWARE Auth intercepted request...");

    // Bạn có thể dùng `pool` tại đây để query kiểm tra token/user nếu muốn
    let _ = &pool;

    // Cho phép request tiếp tục đi tới Handler tiếp theo
    let res = next.run(req).await;

    Ok(res)
}
