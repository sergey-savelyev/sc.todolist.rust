use chrono::Utc;
use domain::models::LogEntity;
use uuid::Uuid;

use crate::{repos::LogsRepository, dtos::{TaskAction, LogEntryDto}};

pub struct LogsService {
    repo: Box<dyn LogsRepository>
}

impl LogsService {
    pub async fn log_task_action(&self, action: TaskAction, entity_id: Option<Uuid>, entity_type: Option<&str>, payload: Option<&str>) {
        let log_entry = LogEntity {
            id: Uuid::new_v4(),
            action: action.as_model(),
            entity_type: entity_type.map_or(None, |s| Some(s.to_string())),
            entity_id: entity_id,
            payload: payload.map_or(None, |s| Some(s.to_string())),
            timestamp: Utc::now().timestamp()
        };

        self.repo.insert(log_entry).await;
    }

    pub async fn get_task_action_log_batch(&self, continuation_token: &str, take: u32, descending: bool) -> (Vec<LogEntryDto>, String) {
        let (entities, ct) = self.repo
            .get_batch_by_entity_type("TaskEntity", continuation_token, take, descending).await;

        (entities.iter().map(|e| LogEntryDto::new(e)).collect(), ct)
    }

    pub async fn get_task_action_log_batch_by_task(&self, task_id: Uuid, continuation_token: &str, take: u32, descending: bool) -> (Vec<LogEntryDto>, String) {
        let (entities, ct) = self.repo
            .get_batch_by_entity(task_id, continuation_token, take, descending).await;

        (entities.iter().map(|e| LogEntryDto::new(e)).collect(), ct)
    }
}