//Router và Extension từ axum, hàm kết nối initialed_db từ dự án, con trỏ thông minh Arc để chia sẻ dữ liệu an toàn giữa các luồng, TcpListener của Tokio để mở cổng mạng, và macro info để ghi log.
use axum::{Extension, Router, routing::get};
use proj_1::dbs::initialed_db;
use std::sync::Arc;
use tokio::net::TcpListener;
use tracing::info;

//Định nghĩa một hàm xử lý (handler) đơn giản trả về chuỗi tĩnh "Hello form Axum" khi người dùng truy cập trang chủ.
async fn hello() -> &'static str {
    "Hello form Axum"
}

#[tokio::main]
async fn main() {
    //Nạp các biến môi trường từ file .env, khởi tạo hệ thống ghi log (tracing), và in thông báo khởi động service.
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();
    info!("Starting service");

    //Lấy chuỗi kết nối từ biến môi trường DATABASE_URL. Nếu không tìm thấy, hệ thống sẽ dùng giá trị mặc định được định nghĩa sẵn.
    let dsn = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:password@localhost:5433/postgres".to_string());

    // Gọi hàm kết nối database với tối đa 5 kết nối đồng thời và in log thông báo thành công.
    let pool = initialed_db(&dsn, 5).await;
    info!("Connect Database successfully!");

    //Khởi tạo ứng dụng Axum (Router), định nghĩa hai đường dẫn (/ gọi hàm hello, /db_check trả về chuỗi ngắn), và bọc kết nối database vào Arc rồi đưa vào Extension để các route có thể dùng chung.
    let app = Router::new()
        .route("/", get(hello))
        .route("/db_check", get(|| async { "Check DB" }))
        .layer(Extension(Arc::new(pool)));

    //Mở một TCP listener lắng nghe trên mọi địa chỉ IP (0.0.0.0) ở cổng 3000.
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    info!("listener on http://localhost:3000");

    //Khởi chạy server Axum với bộ định tuyến và listener đã thiết lập ở trên.
    axum::serve(listener, app).await.unwrap();
}
