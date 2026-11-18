//: Import kiểu dữ liệu PgPool (đối tượng quản lý pool kết nối PostgreSQL) và PgPoolOptions (công cụ cấu hình thông số kết nối) từ thư viện sqlx.
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;

//Định nghĩa một hàm bất đồng bộ (async fn) tên là initialed_db, nhận vào chuỗi kết nối dsn và số lượng kết nối tối đa max_conns, sau đó trả về một PgPool.
pub async fn initialed_db(dsn: &str, max_conns: u32) -> PgPool {
    //Khởi tạo cấu hình pool, thiết lập giới hạn kết nối tối đa, thực hiện kết nối bất đồng bộ tới cơ sở dữ liệu (.connect(dsn).await). Nếu kết nối thất bại, chương trình sẽ dừng lại ngay lập tức và in ra lỗi Cannot connect database
    let db = PgPoolOptions::new()
        .max_connections(max_conns)
        .connect(dsn)
        .await
        .expect("Cannot connect database");

    //Tự động tìm và chạy các file migration (thường nằm ở thư mục migrations/) để cập nhật cấu trúc bảng mới nhất cho database. Nếu chạy migration lỗi, ứng dụng sẽ dừng lại.
    sqlx::migrate!()
        .run(&db)
        .await
        .expect("Cannot migrate database");

    db
}
