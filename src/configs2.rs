use dotenvy::var; //Import hàm var từ thư viện dotenvy để đọc trực tiếp các biến môi trường (environment variables) trong hệ điều hành hoặc file .env.
use serde::Deserialize; //Import trait Deserialize của thư viện serde, cho phép tự động ánh xạ (map) dữ liệu từ nguồn cấu hình (như biến môi trường) vào các cấu trúc struct của Rust.

use crate::errors::AppError; //Import kiểu dữ liệu lỗi tùy chỉnh AppError của dự án để dùng cho kết quả trả về (Result<Self, AppError>).

// : Gắn các macro tự động cho struct:
//Deserialize: Cho phép parse dữ liệu thành struct này.
//Debug: Cho phép in cấu trúc ra màn hình bằng lệnh println!("{:?}", config).
//Clone: Cho phép nhân bản (copy) giá trị của struct khi cần truyền đi nhiều nơi.
#[derive(Deserialize, Debug, Clone)]
pub struct WebConfig {
    //Chứa thông tin cấu hình cho Web Server
    pub addr: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Postgres {
    pub dsn: String,
    pub max_conns: u32,
}

// AppConfig: Cấu hình chung và chuẩn hóa cuối cùng mà toàn bộ ứng dụng sẽ sử dụng (gồm web và postgres).
#[derive(Deserialize, Debug, Clone)]
pub struct AppConfig {
    pub web: WebConfig,
    pub postgres: Postgres,
}

//DevConfig / ProdConfig: Cấu hình riêng cho môi trường Development và Production (cấu trúc bên trong giống nhau nhưng có thể phục vụ các nguồn dữ liệu khác nhau).
#[derive(Deserialize, Debug, Clone)]
pub struct DevConfig {
    pub web: WebConfig,
    pub postgres: Postgres,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ProdConfig {
    pub web: WebConfig,
    pub postgres: Postgres,
}

//DevEnv / ProdEnv: Các struct bao bọc bên ngoài (wrapper) chứa trường app (kiểu DevConfig hoặc ProdConfig). Cấu trúc này dùng để khớp với cách đặt tên phân cấp của biến môi trường khi nạp qua thư viện config
#[derive(Deserialize, Debug, Clone)]
pub struct DevEnv {
    pub app: DevConfig,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ProdEnv {
    pub app: ProdConfig,
}

impl AppConfig {
    //pub fn from_env() -> Result<Self, AppError>: Định nghĩa hàm tĩnh from_env có nhiệm vụ quét biến môi trường và trả về AppConfig hoặc lỗi AppError.
    pub fn from_env() -> Result<Self, AppError> {
        //match var("ENV").as_deref(): Đọc giá trị của biến môi trường tên là ENV. Chuyển đổi kết quả sang kiểu tham chiếu chuỗi &str (as_deref()) để dễ so sánh.
        match var("ENV").as_deref() {
            Ok("prod") => {
                //config::Config::builder(): Khởi tạo trình xây dựng cấu hình từ thư viện config.
                //.add_source(...): Thêm nguồn nạp dữ liệu là biến môi trường của hệ thống.
                //.separator("__"): Quy định dấu phân cách giữa các cấp độ struct là hai dấu gạch dưới (__). Ví dụ: biến môi trường APP__WEB__ADDR sẽ tự động ánh xạ vào app.web.addr.
                //.build()?: Tổng hợp lại các nguồn cấu hình (nếu lỗi sẽ trả về ? ra ngoài hàm).
                //.try_deserialize::<ProdEnv>()?: Giải mã (deserialize) dữ liệu vừa gom được vào cấu trúc ProdEnv.
                let config = config::Config::builder()
                    .add_source(config::Environment::default().separator("__"))
                    .build()?
                    .try_deserialize::<ProdEnv>()?;
                Ok(AppConfig {
                    web: config.app.web,
                    postgres: config.app.postgres,
                })
            }
            _ => {
                let config = config::Config::builder()
                    .add_source(config::Environment::default().separator("__"))
                    .build()?
                    .try_deserialize::<DevEnv>()?;
                Ok(AppConfig {
                    web: config.app.web,
                    postgres: config.app.postgres,
                })
            }
        }
    }
}
