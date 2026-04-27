use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};
use uuid::Uuid;

#[derive(Serialize, FromRow)]
pub struct Employee {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub is_available: bool,
    pub days_off_allowance: i32,
    pub days_off_used: i32,
    pub created_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Type)]
#[sqlx(type_name = "time_off_status")]
pub enum Status {
    pending,
    approved,
    rejected,
}

#[derive(Serialize, FromRow)]
pub struct TimeOffRequest {
    pub id: Uuid,
    pub employee_id: Uuid,
    pub reason: String,
    pub status: Status,
    pub days: i32,
    pub start_date: DateTime<Utc>,
    pub finish_date: DateTime<Utc>,
}

#[derive(Deserialize)]
pub struct CreateTimeOffRequestPayload {
    pub reason: String,
    pub days: i32,
    pub start_date: DateTime<Utc>,
    pub finish_date: DateTime<Utc>,
}

#[derive(Deserialize)]
pub struct UpdateTimeOffStatusPayload {
    pub status: Status,
}
