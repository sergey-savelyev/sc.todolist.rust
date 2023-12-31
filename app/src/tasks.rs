use std::sync::Arc;

use domain::models::TaskEntity;

use chrono::Utc;
use uuid::Uuid;

use crate::{dtos::{TaskFullDto, UpsertTaskDto, TaskSearchDto, TaskDetailedDto, TaskAction}, repos::TaskRepository, errors::Error, logs::LogService};

pub struct TaskService {
    repo: Arc<dyn TaskRepository>,
    log_service: Arc<LogService>
}

impl TaskService {
    pub fn new(repo: Arc<dyn TaskRepository>, log_service: Arc<LogService>) -> TaskService {
        TaskService { repo, log_service }
    }

    pub async fn get_root_task_batch(&self, take: i32, continuation_token: &str, sort_by: &str, descending: bool) -> (Vec<TaskDetailedDto>, String) {
        let (entities, ct) = self.repo.get_root_task_batch(take, continuation_token, sort_by, descending).await;

        (entities.iter().map(|e| TaskDetailedDto::new(e)).collect(), ct)
    }

    pub async fn get_task(&self, id: Uuid) -> Result<TaskFullDto, Error> {
        let entity = self.repo.get_by_id(id).await?;
        let root_entity = match entity.root_task_id {
            Some(root_id) => Some(self.repo.get_by_id(root_id).await.unwrap()),
            None => None
        };

        let subtasks = self.repo.get_subtasks(id).await;
        
        Ok(TaskFullDto::new(&entity, root_entity.as_ref(), &subtasks))
    }

    pub async fn create_task(&self, details: &UpsertTaskDto) -> Result<Uuid, Error> {
        let id = Uuid::new_v4();
        let entity = TaskEntity {
            id,
            root_task_id: None,
            summary: details.summary.clone(),
            description: details.description.clone(),
            create_date: Utc::now(),
            due_date: details.due_date,
            priority: details.priority.as_model(),
            status: details.status.as_model()
        };

        self.repo.insert(entity).await?;
        self.log_service.log_task_action(TaskAction::Create, Some(id), Some("TaskEntity"), None).await;

        Ok(id)
    }

    pub async fn update_task(&self, task_id: Uuid, details: &UpsertTaskDto) -> Result<(), Error>{
        self.repo
            .update_task(task_id, details.summary.as_str(), details.description.as_deref(), details.due_date, details.priority.as_model(), details.status.as_model())
            .await?;

        self.log_service.log_task_action(TaskAction::Update, Some(task_id), Some("TaskEntity"), None).await;

        Ok(())
    }

    pub async fn update_task_root(&self, task_id: Uuid, new_root_id: Option<Uuid>) -> Result<(), Error> {
        if task_id == new_root_id.unwrap_or_default() {
            return Err(Error::invalid_root_binding("Can't bind task to itself"));
        }

        if let Some(new_root_id_unwrapped) = new_root_id {
            let flat_subtask_ids = self.repo.get_all_subtasks_recursive(task_id).await;
            if flat_subtask_ids.contains(&new_root_id_unwrapped) {
                return Err(Error::invalid_root_binding("Can't bind task to its subtask"));
            }
        }

        self.repo.update_task_root(task_id, new_root_id).await?;
        self.log_service.log_task_action(TaskAction::RootChanged, Some(task_id), Some("TaskEntity"), None).await;

        Ok(())
    }

    pub async fn delete_task(&self, task_id: Uuid) -> Result<(), Error> {
        self.repo.delete(task_id).await?;
        self.log_service.log_task_action(TaskAction::Delete, Some(task_id), Some("TaskEntity"), None).await;

        Ok(())
    }

    pub async fn search_tasks(&self, phrase: &str, take: i32, continuation_token: &str) -> (Vec<TaskSearchDto>, String) {
        let (entities, ct) = self.repo.search_tasks(phrase, take, continuation_token).await;
        (entities.iter().map(|e| TaskSearchDto::new(e)).collect(), ct)
    }
}