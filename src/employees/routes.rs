use crate::employees::handlers::{get_employee_list, get_time_off_request_by_employee};
use axum::{Router, routing::get};
use sqlx::{Pool, Postgres};

pub fn employees_routes() -> Router<Pool<Postgres>> {
    Router::new()
        .route("/", get(get_employee_list))
        .route("/time_off_requests", get(get_time_off_request_by_employee))
}
