use crate::employees::models::{CreateTimeOffRequestPayload, Employee, TimeOffRequest};
use axum::{
    Json,
    extract::{Path, State},
};
use sqlx::{Pool, Postgres, types::Uuid};

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

pub async fn get_time_off_list(
    State(pool): State<Pool<Postgres>>,
) -> Result<Json<Vec<TimeOffRequest>>, String> {
    let query = r#"
        SELECT * FROM time_off_requests
    "#;
    let items = sqlx::query_as::<_, TimeOffRequest>(query)
        .fetch_all(&pool)
        .await
        .map_err(|error| error.to_string())?;

    Ok(Json(items))
}

pub async fn create_time_off_request(
    State(pool): State<Pool<Postgres>>,
    Path(raw_employee_id): Path<String>,
    Json(payload): Json<CreateTimeOffRequestPayload>,
) -> Result<(), String> {
    let employee_id = Uuid::parse_str(&raw_employee_id).map_err(|error| error.to_string())?;

    let query = r#"
        INSERT INTO time_off_requests (employee_id, reason, days, start_date, finish_date)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING *
    "#;
    let item = sqlx::query_as::<_, TimeOffRequest>(query)
        .bind(employee_id)
        .bind(payload.reason)
        .bind(payload.days)
        .bind(payload.start_date)
        .bind(payload.finish_date)
        .fetch_one(&pool)
        .await
        .map_err(|error| error.to_string())?;

    Ok(())
}
