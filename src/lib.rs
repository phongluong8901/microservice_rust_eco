pub mod configs2;
pub mod dbs;
pub mod errors;

use axum::Json;
pub use errors::AppError;

pub type AppResult<T> = std::result::Result<T, AppError>;
pub type AppResultJson<T> = std::result::Result<Json<T>, AppError>;
