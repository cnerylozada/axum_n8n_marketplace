use crate::employees::models::{Employee, Status, TimeOffRequest};
use axum::{Json, extract::State};
use chrono::{Duration, Utc};
use sqlx::{Pool, Postgres};

pub async fn get_employee_list(
    State(pool): State<Pool<Postgres>>,
) -> Result<Json<Vec<Employee>>, String> {
    let query = r#"
        SELECT * FROM employees
    "#;
    let employee_list = sqlx::query_as::<_, Employee>(query)
        .fetch_all(&pool)
        .await
        .map_err(|error| error.to_string())?;

    Ok(Json(employee_list))
}

pub async fn get_time_off_request_by_employee() -> Result<Json<Vec<TimeOffRequest>>, String> {
    let items = vec![TimeOffRequest {
        reason: String::from("xxx"),
        status: Status::pending,
        days: 7,
        start_date: Utc::now(),
        finish_date: Utc::now() + Duration::days(2),
    }];

    Ok(Json(items))
}
