use std::sync::Arc;

use app::{errors::Error, dtos::UpsertTaskDto};
use axum::{
    response::IntoResponse, 
    http::StatusCode, 
    Json, 
    extract::{
        State, 
        Path, Query
    }
};
use infrastructure::ServiceProvider;
use serde_json::{json, Value};

use crate::view::{Pagination, BatchResponse, CreateTaskResponse, TaskRootChangeRequest};

pub async fn get_task(
    Path(id): Path<uuid::Uuid>,
    State(services): State<Arc<ServiceProvider>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let result = services.task_service().get_task(id).await;

    match result {
        Ok(task) => {
            let task_response = json!(task);

            return Ok(Json(task_response));
        }

        Err(Error::EntityNotFound(err)) => {
            let error_response = serde_json::json!({
                "status": "fail",
                "message": err
            });
            return Err((StatusCode::NOT_FOUND, Json(error_response)));
        }
        
        Err(e) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"status": "error","message": format!("{:?}", e)})),
            ));
        }
    };
}

pub async fn get_tasks_batch(
    pagination: Query<Pagination>,
    State(services): State<Arc<ServiceProvider>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let (batch, continuation_token) = services.task_service()
        .get_root_task_batch(pagination.take(), &pagination.skip().to_string(), &pagination.sort_by().unwrap_or("CreateDate"), pagination.descending_sort().unwrap_or(false))
        .await;

    Ok(Json(json!(BatchResponse::new(batch, continuation_token))))
}

pub async fn search_tasks(
    phrase: Path<String>,
    pagination: Query<Pagination>,
    State(services): State<Arc<ServiceProvider>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let (batch, continuation_token) = services.task_service()
        .search_tasks(&phrase, pagination.take(), &pagination.skip().to_string())
        .await;

    Ok(Json(json!(BatchResponse::new(batch, continuation_token))))
}

pub async fn create_task(
    State(services): State<Arc<ServiceProvider>>,
    Json(payload): Json<Value>
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    if let Ok(task_details)  = serde_json::from_value::<UpsertTaskDto>(payload) {
        match services.task_service().create_task(&task_details).await {
            Ok(created_task_id) => return Ok(Json(json!(CreateTaskResponse::new(created_task_id)))),

            Err(Error::DbError(message)) => {
                let error_response = serde_json::json!({
                    "status": "fail",
                    "message": message
                });

                return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)));
            },

            Err(err) => {
                let error_response = serde_json::json!({
                    "status": "fail",
                    "message": format!("{:?}", err)
                });

                return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)));
            }
        }
    }

    let error_response = serde_json::json!({
        "status": "fail",
        "message": "Invalid input"
    });

    return Err((StatusCode::BAD_REQUEST, Json(error_response)));
}

pub async fn update_task(
    Path(id): Path<uuid::Uuid>,
    State(services): State<Arc<ServiceProvider>>,
    Json(payload): Json<Value>
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    if let Ok(task_details)  = serde_json::from_value::<UpsertTaskDto>(payload) {
        match services.task_service().update_task(id, &task_details).await {
            Ok(()) => return Ok(StatusCode::NO_CONTENT),

            Err(Error::EntityNotFound(message)) => {
                let error_response = serde_json::json!({
                    "status": "fail",
                    "message": message
                });

                return Err((StatusCode::NOT_FOUND, Json(error_response)))
            },

            Err(err) => {
                let error_response = serde_json::json!({
                    "status": "fail",
                    "message": format!("{:?}", err)
                });

                return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)))
            }
        }
    }

    let error_response = serde_json::json!({
        "status": "fail",
        "message": "Invalid input"
    });
    return Err((StatusCode::BAD_REQUEST, Json(error_response)));
}

pub async fn change_task_root(
    Path(id): Path<uuid::Uuid>,
    State(services): State<Arc<ServiceProvider>>,
    Json(payload): Json<Value>
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    if let Ok(change_root_request)  = serde_json::from_value::<TaskRootChangeRequest>(payload) {
        match services.task_service().update_task_root(id, change_root_request.root_id()).await {
            
            Ok(()) => return Ok(StatusCode::NO_CONTENT),

            Err(Error::EntityNotFound(message)) => {
                let error_response = serde_json::json!({
                    "status": "fail",
                    "message": message
                });

                return Err((StatusCode::NOT_FOUND, Json(error_response)))
            },

            Err(err) => {
                let error_response = serde_json::json!({
                    "status": "fail",
                    "message": format!("{:?}", err)
                });

                return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)))
            }
        }
    }

    let error_response = serde_json::json!({
        "status": "fail",
        "message": "Invalid input"
    });
    return Err((StatusCode::BAD_REQUEST, Json(error_response)));
}

pub async fn delete_task(
    Path(id): Path<uuid::Uuid>,
    State(services): State<Arc<ServiceProvider>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    match services.task_service().delete_task(id).await {
        
        Ok(()) => return Ok(StatusCode::NO_CONTENT),

        Err(Error::EntityNotFound(message)) => {
            let error_response = serde_json::json!({
                "status": "fail",
                "message": message
            });

            return Err((StatusCode::NOT_FOUND, Json(error_response)))
        },
        
        Err(err) => {
            let error_response = serde_json::json!({
                "status": "fail",
                "message": format!("{:?}", err)
            });

            return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)))
        }
    }
}