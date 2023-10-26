use std::sync::Arc;

use axum::{extract::{Query, State, Path}, response::IntoResponse, Json, http::StatusCode};
use infrastructure::ServiceProvider;
use serde_json::json;
use uuid::Uuid;

use crate::view::{Pagination, BatchResponse};

pub async fn get_task_logs(
    Path(id): Path<Uuid>,
    pagination: Query<Pagination>,
    State(services): State<Arc<ServiceProvider>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let (batch, continuation_token) = services.log_service()
        .get_task_action_log_batch_by_task(id, &pagination.skip().to_string(), pagination.take(), pagination.descending().unwrap_or(false))
        .await;

    Ok(Json(json!(BatchResponse::new(batch, continuation_token))))
}

pub async fn get_all_logs(
    pagination: Query<Pagination>,
    State(services): State<Arc<ServiceProvider>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let (batch, continuation_token) = services.log_service()
        .get_task_action_log_batch(&pagination.skip().to_string(), pagination.take(), pagination.descending().unwrap_or(false))
        .await;

    Ok(Json(json!(BatchResponse::new(batch, continuation_token))))
}