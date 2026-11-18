//Router và Extension từ axum, hàm kết nối initialed_db từ dự án, con trỏ thông minh Arc để chia sẻ dữ liệu an toàn giữa các luồng, TcpListener của Tokio để mở cổng mạng, và macro info để ghi log.
use axum::{Extension, Router, routing::get};
#[allow(unused_imports)]
use proj_1::configs::{get_dsn, get_max_conns, get_port};
use proj_1::{configs2::AppConfig, dbs::initialed_db};
use std::sync::Arc;
use tokio::net::TcpListener;
use tracing::info;

//Định nghĩa một hàm xử lý (handler) đơn giản trả về chuỗi tĩnh "Hello form Axum" khi người dùng truy cập trang chủ.
async fn hello() -> &'static str {
    "Hello form Axum"
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();
    info!("Starting service");

    // ==========================================
    // CACH 1: Dung configs.rs
    // ==========================================
    // let pool = initialed_db(get_dsn().as_str(), get_max_conns()).await;
    // info!("Connect Database successfully!");

    // ==========================================
    // CACH 2: Dung configs2.rs (AppConfig)
    // ==========================================
    let cfg = AppConfig::from_env().expect("Cannot load config from environment");
    let pool = initialed_db(&cfg.postgres.dsn, cfg.postgres.max_conns).await;
    info!("Connect Database successfully!");

    //Khởi tạo ứng dụng Axum (Router), định nghĩa hai đường dẫn (/ gọi hàm hello, /db_check trả về chuỗi ngắn), và bọc kết nối database vào Arc rồi đưa vào Extension để các route có thể dùng chung.
    let app = Router::new()
        .route("/", get(hello))
        .route("/db_check", get(|| async { "Check DB" }))
        .layer(Extension(Arc::new(pool)));

    // ==========================================
    // CACH 1: Mo cong tu configs.rs
    // ==========================================
    // let addr = get_port();
    // info!("Binding server to {}", addr);
    // let listener = TcpListener::bind(&addr).await.unwrap_or_else(|err| {
    //     panic!(
    //         "Khong the mo cong {}: {}. Hay doi sang cong khac trong file .env!",
    //         addr, err
    //     );
    // });

    // ==========================================
    // CACH 2: Mo cong tu configs2.rs
    // ==========================================
    let addr = cfg.web.addr;
    info!("Binding server to {}", addr);

    let listener = TcpListener::bind(&addr).await.unwrap_or_else(|err| {
        panic!(
            "Khong the mo cong {}: {}. Hay doi sang cong khac trong file .env!",
            addr, err
        );
    });

    info!("Server is running on http://{}", addr);

    //Khởi chạy server Axum với bộ định tuyến và listener đã thiết lập ở trên.
    axum::serve(listener, app).await.unwrap();
}
