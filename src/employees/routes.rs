use crate::employees::handlers::{create_time_off_request, get_employee_list, get_time_off_list};
use axum::{
    Router,
    routing::{get, post},
};
use sqlx::{Pool, Postgres};

pub fn employees_routes() -> Router<Pool<Postgres>> {
    Router::new()
        .route("/", get(get_employee_list))
        .route("/time_off_requests", get(get_time_off_list))
        .route(
            "/{employee_id}/time_off_requests",
            post(create_time_off_request),
        )
}
