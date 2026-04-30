use crate::employees::models::Employee;
use axum::{
    Json,
    extract::{Path, State},
};
use sqlx::{Pool, Postgres};
use uuid::Uuid;

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

pub async fn get_employee_by_id(
    State(pool): State<Pool<Postgres>>,
    Path(employee_id): Path<String>,
) -> Result<Json<Employee>, String> {
    let employee_id = Uuid::parse_str(&employee_id).map_err(|error| error.to_string())?;

    let query = r#"
        SELECT * FROM employees
        WHERE id = $1
    "#;

    let employee = sqlx::query_as::<_, Employee>(query)
        .bind(employee_id)
        .fetch_one(&pool)
        .await
        .map_err(|error| error.to_string())?;

    Ok(Json(employee))
}
