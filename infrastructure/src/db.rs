use app::{repos::{TaskRepository, LogRepository}, errors::Error};
use domain::models::{TaskEntity, LogEntity};

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{mysql::{MySqlPool, MySqlRow}, Row};
use uuid::Uuid;

use crate::convert;

pub struct TaskStorage {
    // At first I intended to use Arc (thread-safe ref count pointer)
    // But then I found that MySqlPool implements an Arc pointer itself :) 
    pool: MySqlPool
}

pub struct LogStorage {
    pool: MySqlPool
}

impl TaskStorage {
    pub fn new(pool: MySqlPool) -> TaskStorage {
        TaskStorage { pool }
    }
}

impl LogStorage {
    pub fn new(pool: MySqlPool) -> LogStorage {
        LogStorage { pool }
    }
}

#[async_trait]
impl TaskRepository for TaskStorage {
    async fn get_by_id(&self, id: Uuid) -> Result<TaskEntity, Error> {
        let result = 
            sqlx::query("SELECT * FROM Tasks WHERE Id = ?")
                .bind(id.to_string())
                .map(|row: MySqlRow| {
                    convert::row_to_task_entity(&row)
                })
                .fetch_optional(&self.pool)
                .await
                .unwrap();

        if let Some(r) = result {
            return Ok(r);
        }

        Err(Error::EntityNotFound(id.to_string()))
    }

    async fn insert(&self, entity: TaskEntity) {
        let _ = 
            sqlx::query("INSERT INTO Tasks (Id, RootTaskId, Summary, Description, CreateDate, DueDate, Priority, Status) VALUES (?, ?, ?, ?, ?, ?, ?, ?)")
                .bind(entity.id.to_string())
                .bind(entity.root_task_id.map_or(String::from("NULL"), |u| u.to_string()))
                .bind(entity.summary)
                .bind(entity.description)
                .bind(entity.create_date)
                .bind(entity.due_date)
                .bind(convert::priority_to_str(entity.priority))
                .bind(convert::status_to_str(entity.status))
                .execute(&self.pool)
                .await;
        }

    async fn delete(&self, id: Uuid) -> Result<(), Error> {
        let affected = 
            sqlx::query("DELETE FROM Tasks WHERE Id = ?")
                .bind(id.to_string())
                .execute(&self.pool)
                .await
                .unwrap()
                .rows_affected();

        if affected > 0 { Ok(()) } else { Err(Error::not_found(id)) }
    }

    async fn get_subtasks(&self, task_id: Uuid) -> Vec<TaskEntity> {
        let result = 
            sqlx::query("SELECT * FROM Tasks WHERE RootTaskId = ?")
                .bind(task_id.to_string())
                .map(|row: MySqlRow| {
                    convert::row_to_task_entity(&row)
                })
                .fetch_all(&self.pool)
                .await;

        result.unwrap_or(vec![])
    }

    async fn get_root_task_batch(&self, take: u32, continuation_token: &str, sort_by: &str, descending: bool) -> (Vec<domain::models::TaskEntity>, String) {
        let sort = if descending { "DESC" } else { "ASC" };
        let skip = continuation_token.parse::<u32>().unwrap();
        let result = 
            sqlx::query("SELECT * FROM Tasks WHERE RootTaskId = NULL ORDER BY ? ? LIMIT ? OFFSET ?")
                .bind(sort_by)
                .bind(sort)
                .bind(take)
                .bind(skip)
                .map(|row: MySqlRow| {
                    convert::row_to_task_entity(&row)
                })
                .fetch_all(&self.pool)
                .await;

        (result.unwrap_or(vec![]), {take + skip}.to_string())
    }

    async fn search_tasks(&self, phrase: &str, take: u32, continuation_token: &str) -> (Vec<domain::models::TaskSearchEntity>, String) {
        let skip = continuation_token.parse::<u32>().unwrap();
        let result = 
            sqlx::query("SELECT * FROM Tasks WHERE Summary LIKE ? OR Description LIKE ? ORDER BY CreateDate LIMIT ? OFFSET ?")
                .bind(format!("%{}%", phrase))
                .bind(format!("%{}%", phrase))
                .bind(take)
                .bind(skip)
                .map(|row: MySqlRow| {
                    convert::row_to_task_search_entity(&row)
                })
                .fetch_all(&self.pool)
                .await;

        (result.unwrap_or(vec![]), {take + skip}.to_string())
    }

    async fn get_all_subtasks_recursive(&self, task_id: Uuid) -> Vec<Uuid> {
        let result = sqlx::query(format!("with recursive cte (Id, RootTaskId) as ( \
select     Id, \
            RootTaskId \
from       todolist.Tasks \
where      RootTaskId = '{}' \
union all \
select     t.Id, \
            t.RootTaskId \
from       todolist.Tasks t \
inner join cte \
        on t.RootTaskId = cte.Id \
) \
select cte.Id as val from cte;", task_id.to_string()).as_str())
            .bind(task_id.to_string())
            .map(|row: MySqlRow| {
                Uuid::parse_str(row.get("val")).unwrap()
            })
            .fetch_all(&self.pool)
            .await;

        result.unwrap_or(vec![])
    }

    async fn update_task_root(&self, task_id: Uuid, new_root_id: Option<Uuid>) -> Result<(), app::errors::Error> {
        let new_root_id = new_root_id.map_or(String::from("NULL"), |u| u.to_string());
        let affected = 
            sqlx::query("UPDATE Tasks SET RootTaskId = ? WHERE Id = ?")
                .bind(new_root_id)
                .bind(task_id.to_string())
                .execute(&self.pool)
                .await
                .unwrap()
                .rows_affected();

        if affected > 0 { Ok(()) } else { Err(Error::EntityNotFound(task_id.to_string())) }
    }

    async fn update_task(&self, id: Uuid, summary: &str, description: Option<&str>, due_date: DateTime<Utc>, priority: domain::enums::TaskPriority, status: domain::enums::TaskStatus) -> Result<(), app::errors::Error> {
        let affected = 
            sqlx::query("UPDATE Tasks SET Summary = ?, Description = ?, DueDate = ?, Priority = ?, Status = ?, WHERE Id = ?")
                .bind(summary)
                .bind(description)
                .bind(due_date)
                .bind(convert::priority_to_str(priority))
                .bind(convert::status_to_str(status))
                .bind(id.to_string())
                .execute(&self.pool)
                .await
                .unwrap()
                .rows_affected();

        if affected > 0 { Ok(()) } else { Err(Error::EntityNotFound(id.to_string())) }
    }
}

#[async_trait]
impl LogRepository for LogStorage {
    async fn insert(&self, entity: LogEntity) {
        let _ = 
            sqlx::query("INSERT INTO Logs (Id, Action, TimestampMsec, EntityId, EntityType, Payload) VALUES (?, ?, ?, ?, ?, ?)")
                .bind(entity.id.to_string())
                .bind(convert::action_to_str(entity.action))
                .bind(entity.timestamp)
                .bind(entity.entity_id.map_or(String::from("NULL"), |u| u.to_string()))
                .bind(entity.entity_type)
                .bind(entity.payload)
                .execute(&self.pool)
                .await;
    }

    async fn get_batch_by_entity_type(&self, entity_type: &str, continuation_token: &str, take: u32, descending: bool) -> (Vec<domain::models::LogEntity>, String) {
        let skip = continuation_token.parse::<u32>().unwrap();
        let sort = if descending { "DESC" } else { "ASC" };

        let result = 
            sqlx::query("SELECT * FROM Logs WHERE EntityType = ? ORDER BY TimestampMsec ? LIMIT ? OFFSET ?")
                .bind(entity_type)
                .bind(sort)
                .bind(take)
                .bind(skip)
                .map(|row: MySqlRow| {
                    convert::row_to_log_entity(&row)
                })
                .fetch_all(&self.pool)
                .await;

        (result.unwrap_or(vec![]), {take + skip}.to_string())
    }

    async fn get_batch_by_entity(&self, entity_id: Uuid, continuation_token: &str, take: u32, descending: bool) -> (Vec<domain::models::LogEntity>, String) {
        let skip = continuation_token.parse::<u32>().unwrap();
        let sort = if descending { "DESC" } else { "ASC" };

        let result = 
            sqlx::query("SELECT * FROM Logs WHERE EntityId = ? ORDER BY TimestampMsec ? LIMIT ? OFFSET ?")
                .bind(entity_id.to_string())
                .bind(sort)
                .bind(take)
                .bind(skip)
                .map(|row: MySqlRow| {
                    convert::row_to_log_entity(&row)
                })
                .fetch_all(&self.pool)
                .await;

        (result.unwrap_or(vec![]), {take + skip}.to_string())
    }
}