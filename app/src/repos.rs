use domain::{models::{LogEntity, TaskEntity, TaskSearchEntity}, enums::{TaskPriority, TaskStatus}};

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::errors::Error;

#[async_trait]
pub trait LogRepository : Send + Sync {
    async fn insert(&self, entity: LogEntity); // Consumes ownership. After insert T should not be used
    async fn get_batch_by_entity_type(&self, entity_type: &str, continuation_token: &str, take: i32, descending: bool) -> (Vec<LogEntity>, String);
    async fn get_batch_by_entity(&self, entity_id: Uuid, continuation_token: &str, take: i32, descending: bool) -> (Vec<LogEntity>, String);
}

#[async_trait]
pub trait TaskRepository : Send + Sync {
    async fn get_by_id(&self, id: Uuid) -> Result<TaskEntity, Error>;
    async fn insert(&self, entity: TaskEntity) -> Result<(), Error>; // Consumes ownership. After insert T should not be used
    async fn delete(&self, id: Uuid) -> Result<(), Error>;
    async fn get_subtasks(&self, task_id: Uuid) -> Vec<TaskEntity>;
    async fn get_root_task_batch(&self, take: i32, continuation_token: &str, sort_by: &str, descending: bool) -> (Vec<TaskEntity>, String);
    async fn search_tasks(&self, phrase: &str, take: i32, continuation_token: &str) -> (Vec<TaskSearchEntity>, String);
    async fn get_all_subtasks_recursive(&self, task_id: Uuid) -> Vec<Uuid>;
    async fn update_task_root(&self, task_id: Uuid, new_root_id: Option<Uuid>) -> Result<(), Error>;
    async fn update_task(&self, id: Uuid, summary: &str, description: Option<&str>, due_date: DateTime<Utc>, priority: TaskPriority, status: TaskStatus) -> Result<(), Error>;
}