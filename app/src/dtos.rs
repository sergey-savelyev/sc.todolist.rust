use domain::{enums, models::LogEntity};
use domain::models::{TaskEntity, TaskSearchEntity};

use chrono::DateTime;
use serde::{Serialize, Deserialize};


#[derive(Serialize)]
pub struct LogEntryDto {
    id: String,
    action: TaskAction,
    timestamp: i64,
    entity_id: Option<String>,
    entity_type: Option<String>,
    payload: Option<String>
}

#[derive(Serialize)]
pub struct CreateTaskResponse {
    task_id: String
}

#[derive(Deserialize)]
pub struct TaskRootChangeRequest {
    pub root_id: String
}

#[derive(Debug, Serialize)]
pub struct TaskBaseDto {
    id: String,
    summary: String,
    priority: TaskPriority,
    status: TaskStatus,
}

#[derive(Debug, Serialize)]
pub struct TaskDetailedDto {
    root_id: Option<String>,

    #[serde(with = "chrono::serde::ts_seconds")]
    create_date: DateTime<chrono::Utc>,

    #[serde(with = "chrono::serde::ts_seconds")]
    due_date: DateTime<chrono::Utc>,

    #[serde(flatten)]
    base: TaskBaseDto,
}

#[derive(Debug, Serialize)]
pub struct TaskFullDto {
    root_task: Option<TaskBaseDto>,
    subtasks: Vec<TaskBaseDto>,
    description: Option<String>,
    
    #[serde(flatten)]
    detailed: TaskDetailedDto,
}

#[derive(Debug, Serialize)]
pub struct TaskSearchDto {
    id: String,
    summary: Option<String>,
    description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpsertTaskDto {
    pub summary: String,
    pub priority: TaskPriority,
    pub status: TaskStatus,
    pub description: Option<String>,
    pub due_date: DateTime<chrono::Utc>
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TaskPriority {
    #[serde(rename = "Low")]
    Low,

    #[serde(rename = "Normal")]
    Normal,

    #[serde(rename = "High")]
    High,

    #[serde(rename = "Urgent")]
    Urgent,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TaskStatus {
    #[serde(rename = "Reserved")]
    Reserved,

    #[serde(rename = "Ongoing")]
    Ongoing,

    #[serde(rename = "Done")]
    Done,

    #[serde(rename = "Pending")]
    Pending,
}

#[derive(Debug, Serialize)]
pub enum TaskAction {
    #[serde(rename = "Create")]
    Create,

    #[serde(rename = "Delete")]
    Delete,

    #[serde(rename = "Update")]
    Update,

    #[serde(rename = "RootChanged")]
    RootChanged,
}

impl TaskPriority {
    fn new(source: &enums::TaskPriority) -> Self {
        match source {
            enums::TaskPriority::Low => TaskPriority::Low,
            enums::TaskPriority::Normal => TaskPriority::Normal,
            enums::TaskPriority::High => TaskPriority::High,
            enums::TaskPriority::Urgent => TaskPriority::Urgent
        }
    }

    pub fn as_model(&self) -> enums::TaskPriority {
        match self {
            TaskPriority::Low => enums::TaskPriority::Low,
            TaskPriority::Normal => enums::TaskPriority::Normal,
            TaskPriority::High => enums::TaskPriority::High,
            TaskPriority::Urgent => enums::TaskPriority::Urgent
        }
    }
}

impl TaskStatus {
    fn new(source: &enums::TaskStatus) -> Self {
        match source {
            enums::TaskStatus::Done => TaskStatus::Done,
            enums::TaskStatus::Ongoing => TaskStatus::Ongoing,
            enums::TaskStatus::Pending => TaskStatus::Pending,
            enums::TaskStatus::Reserved => TaskStatus::Reserved
        }
    }

    pub fn as_model(&self) -> enums::TaskStatus {
        match self {
            TaskStatus::Done => enums::TaskStatus::Done,
            TaskStatus::Ongoing => enums::TaskStatus::Ongoing,
            TaskStatus::Pending => enums::TaskStatus::Pending,
            TaskStatus::Reserved => enums::TaskStatus::Reserved
        }
    }
}

impl TaskAction {
    fn new(source: &enums::TaskAction) -> Self {
        match source {
            enums::TaskAction::Create => TaskAction::Create,
            enums::TaskAction::Delete => TaskAction::Delete,
            enums::TaskAction::Update => TaskAction::Update,
            enums::TaskAction::RootChanged => TaskAction::RootChanged
        }
    }

    pub fn as_model(&self) -> enums::TaskAction {
        match self {
            TaskAction::Create => enums::TaskAction::Create,
            TaskAction::Delete => enums::TaskAction::Delete,
            TaskAction::Update => enums::TaskAction::Update,
            TaskAction::RootChanged => enums::TaskAction::RootChanged
        }
    }
}

impl TaskBaseDto {
    pub fn new(entity: &TaskEntity) -> Self {
        TaskBaseDto {
            id: entity.id.to_string(),
            summary: entity.summary.clone(),
            priority: TaskPriority::new(&entity.priority),
            status: TaskStatus::new(&entity.status)
        }
    }
}

impl TaskDetailedDto {
    pub fn new(entity: &TaskEntity) -> Self {
        TaskDetailedDto {
            root_id: entity.root_task_id.map_or(None, |u| Some(u.to_string())),
            create_date: entity.create_date,
            due_date: entity.due_date,
            base: TaskBaseDto::new(entity)
        }
    }
}

impl TaskFullDto {
    pub fn new(entity: &TaskEntity, root_entity: Option<&TaskEntity>, subtasks: &Vec<TaskEntity>) -> Self {
        TaskFullDto {
            root_task: root_entity.map_or(None, |e| Some(TaskBaseDto::new(e))),
            subtasks: subtasks.iter().map(|e| TaskBaseDto::new(e)).collect(),
            description: entity.description.clone(),
            detailed: TaskDetailedDto::new(entity)
        }
    }
}

impl LogEntryDto {
    pub fn new(entity: &LogEntity) -> Self {
        LogEntryDto { 
            id: entity.id.to_string(),
            action: TaskAction::new(&entity.action),
            timestamp: entity.timestamp,
            entity_id: entity.entity_id.map_or(None, |uuid| Some(uuid.to_string())),
            entity_type: entity.entity_type.clone(),
            payload: entity.payload.clone()
        }
    }
}

impl TaskSearchDto {
    pub fn new(entity: &TaskSearchEntity) -> Self {
        TaskSearchDto { 
            id: entity.id.to_string(),
            summary: entity.summary.clone(),
            description: entity.description.clone()
        }
    }
}