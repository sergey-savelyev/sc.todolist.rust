use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::enums;

pub struct TaskEntity {
    pub id: Uuid,
    pub root_task_id: Option<Uuid>,
    pub summary: String,
    pub description: Option<String>,
    pub create_date: DateTime<Utc>,
    pub due_date: DateTime<Utc>,
    pub priority: enums::TaskPriority,
    pub status: enums::TaskStatus,
}

pub struct TaskSearchEntity {
    pub id: Uuid,
    pub summary: Option<String>,
    pub description: Option<String>
}

pub struct LogEntity {
    pub id: Uuid,
    pub action: enums::TaskAction,
    pub timestamp: i64,
    pub entity_id: Option<Uuid>,
    pub entity_type: Option<String>,
    pub payload: Option<String>
}