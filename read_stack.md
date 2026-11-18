# --- Arc
1. Arc
Arc (viết tắt của Atomic Reference Counted) là một con trỏ thông minh (smart pointer) trong Rust, dùng để quản lý quyền sở hữu dữ liệu sao cho nhiều luồng (threads) có thể cùng đọc chung một dữ liệu một cách an toàn mà không bị tranh chấp bộ nhớ.

2. Đặc điểm chính của Arc
Chia sẻ quyền sở hữu đa luồng (Thread-safe): Khác với Rc (chỉ dùng được trong 1 luồng đơn), Arc sử dụng các thao tác nguyên tử (atomic operations) để tăng giảm bộ đếm tham số. Do đó, bạn hoàn toàn có thể truyền Arc qua lại giữa các luồng Tokio bất đồng bộ.

Bộ đếm tham chiếu (Reference Counting): Mỗi khi bạn gọi .clone() trên một Arc, nó không nhân bản (copy) toàn bộ dữ liệu bên trong, mà chỉ tăng bộ đếm tham số lên 1. Khi một biến Arc ra khỏi phạm vi (scope) và bị hủy, bộ đếm giảm đi 1. Khi bộ đếm về 0, dữ liệu thực sự trên RAM sẽ được giải phóng tự động.

Chỉ đọc (Read-only / Immutable by default): Bản thân Arc chỉ cung cấp quyền chia sẻ quyền đọc. Mặc định bạn không thể sửa đổi dữ liệu bên trong Arc trừ khi kết hợp thêm các cơ chế khóa đồng bộ như Mutex hoặc RwLock (nếu cần ghi).

3. 
Tại sao lại cần Arc bọc lấy PgPool?
PgPool bản chất đã là một con trỏ thông minh chia sẻ kết nối: Bản thân sqlx::PgPool bên trong đã tự quản lý một pool các kết nối cơ sở dữ liệu và tự động thread-safe. Tuy nhiên, khi bạn muốn truyền nó vào hệ thống định tuyến của Axum để các HTTP Handler khác nhau có thể sử dụng chung, Axum yêu cầu các kiểu dữ liệu truyền qua Extension hoặc State phải thỏa mãn trait Clone và đặc biệt là phải an toàn khi chia sẻ giữa nhiều luồng.

Tiết kiệm tài nguyên: Thay vì mở một kết nối database mới cho mỗi request (gây chậm hệ thống và cạn kiệt kết nối của Postgres), bạn chỉ khởi tạo một PgPool duy nhất khi ứng dụng khởi động, sau đó dùng Arc::clone(pool) để nhân bản con trỏ (chỉ tốn chi phí copy một địa chỉ con trỏ và tăng bộ đếm lên 1) rồi đưa vào Axum.

# ---