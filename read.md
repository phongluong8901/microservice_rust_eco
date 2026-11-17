# --- src
https://github.com/jayden-dang/rust-microservices-monorepo

# --- stack
axum - http method
Tokio - Concurrency (async, multi thread)
slqx - seaORM: Query
    sqlx: raw query -> perfomance cao
    ORM
        seaORM: de -> syntax -> newbie
        Diesel: Sqlx <> seaORM -> trung binh kho -> syntax macros -> intermedia
redis
Opentelemetry - tracing
gRpc - tonic/protobuf
AWS (Ec2 - S3)
github actions
rabbitmq (queue, notification)

# --- setup
# 1. Tự tạo thư mục và cấu trúc dự án Rust mới tên là proj_1
cargo new proj_1

cd proj_1

- Cài đặt công cụ tự động hóa tác vụ (task runner) cho Rust. Công cụ này đọc các cấu hình trong file Makefile.toml của dự án để chạy các chuỗi lệnh phức tạp một cách gọn nhẹ.
cargo install cargo-make

- Cài đặt công cụ chạy kiểm thử (test runner) thế hệ mới cho Rust, giúp chạy các bài test nhanh hơn và hiển thị kết quả trực quan hơn so với lệnh cargo test mặc định.
cargo install cargo-nextest

- Cài đặt công cụ theo dõi sự thay đổi của mã nguồn. Khi bạn sửa và lưu file code, công cụ này sẽ tự động biên dịch lại hoặc chạy lại dự án mà không cần gõ lệnh thủ công.
cargo install cargo-watch

- --- tom gon
cargo install cargo-make cargo-nextest cargo-watch
cargo install --locked cargo-nextest

- Gọi cargo-make để thực thi quy trình biên dịch (build) dự án theo đúng kịch bản mà tác giả dự án đã định nghĩa sẵn.
cargo make build

- Gọi cargo-make để khởi chạy ứng dụng sau khi đã biên dịch xong.
cargo make run

# --- install
# 1. Web framework (định tuyến API, xử lý HTTP)
cargo add axum

# 2. Async runtime (bắt buộc để chạy axum bất đồng bộ)
cargo add tokio --features full

# 3. Serialize / Deserialize JSON (đọc và trả về dữ liệu JSON)
cargo add serde --features derive
cargo add serde_json

# 4. Logging & Tracing (ghi log theo chuẩn microservice)
cargo add tracing tracing-subscriber

# 5. Đọc biến môi trường từ file .env
cargo add dotenvy


# --- generate code

# --- run
# 1. Biên dịch mã nguồn cơ bản (kiểm tra lỗi và tạo file thực thi dev)
cargo build

# 2. Chạy tác vụ build chuẩn của dự án theo cấu hình trong Makefile.toml
cargo make build

# 3. Khởi động môi trường lập trình (chạy kịch bản dev định nghĩa trong Makefile.toml)
cargo make dev

# 4. Theo dõi thay đổi file code và tự động biên dịch + chạy lại server (hot-reload)
cargo watch -x run

# --- docker

# --- curl api test
curl.exe http://localhost:3000