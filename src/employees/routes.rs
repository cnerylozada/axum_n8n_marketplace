use crate::employees::handlers::employees::{get_employee_by_id, get_employee_list};
use crate::employees::handlers::time_off_requests::{
    create_time_off_request, get_time_off_list, update_time_off_request_status,
};
use axum::{
    Router,
    routing::{get, patch, post},
};
use sqlx::{Pool, Postgres};

pub fn employees_routes() -> Router<Pool<Postgres>> {
    Router::new()
        .route("/", get(get_employee_list))
        .route("/{employee_id}", get(get_employee_by_id))
        .route("/time_off_requests", get(get_time_off_list))
        .route(
            "/{employee_id}/time_off_requests",
            post(create_time_off_request),
        )
        .route(
            "/{employee_id}/time_off_requests/{request_id}",
            patch(update_time_off_request_status),
        )
}
