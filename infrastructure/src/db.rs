use app::{repos::{TaskRepository, LogRepository}, errors::Error};
use domain::models::{TaskEntity, LogEntity};

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{postgres::{PgPool, PgRow}, Row, migrate::Migrator};
use uuid::Uuid;

use crate::convert;

static MIGRATOR: Migrator = sqlx::migrate!("../migrations");

pub struct DbMigrator {
    pool: PgPool
}

impl DbMigrator {
    pub fn new(pool: PgPool) -> DbMigrator {
        DbMigrator { pool }
    }

    pub async fn migrate(&self) -> Result<(), sqlx::migrate::MigrateError> {
        Ok(MIGRATOR.run(&self.pool).await?)
    }
}

pub struct TaskStorage {
    // At first I intended to use Arc (thread-safe ref count pointer)
    // But then I found that MySqlPool implements an Arc pointer itself :) 
    pool: PgPool
}

pub struct LogStorage {
    pool: PgPool
}

impl TaskStorage {
    pub fn new(pool: PgPool) -> TaskStorage {
        TaskStorage { pool }
    }
}

impl LogStorage {
    pub fn new(pool: PgPool) -> LogStorage {
        LogStorage { pool }
    }
}

#[async_trait]
impl TaskRepository for TaskStorage {
    async fn get_by_id(&self, id: Uuid) -> Result<TaskEntity, Error> {
        let result = 
            sqlx::query("SELECT * FROM Tasks WHERE Id = $1")
                .bind(id)
                .map(|row: PgRow| {
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

    async fn insert(&self, entity: TaskEntity) -> Result<(), Error> {
        let result = 
            sqlx::query("INSERT INTO Tasks (Id, Summary, Description, CreateDate, DueDate, Priority, Status) VALUES ($1, $2, $3, $4, $5, $6, $7)")
                .bind(entity.id)
                .bind(entity.summary)
                .bind(entity.description)
                .bind(entity.create_date)
                .bind(entity.due_date)
                .bind(convert::priority_to_i16(entity.priority))
                .bind(convert::status_to_i16(entity.status))
                .execute(&self.pool)
                .await;

            if let Err(err) = result {
                return Err(Error::DbError(err.to_string()));
            }

            Ok(())
        }

    async fn delete(&self, id: Uuid) -> Result<(), Error> {
        let affected = 
            sqlx::query("DELETE FROM Tasks WHERE Id = $1")
                .bind(id)
                .execute(&self.pool)
                .await
                .unwrap()
                .rows_affected();

        if affected > 0 { Ok(()) } else { Err(Error::not_found(id)) }
    }

    async fn get_subtasks(&self, task_id: Uuid) -> Vec<TaskEntity> {
        let result = 
            sqlx::query("SELECT * FROM Tasks WHERE RootTaskId = $1")
                .bind(task_id)
                .map(|row: PgRow| {
                    convert::row_to_task_entity(&row)
                })
                .fetch_all(&self.pool)
                .await;

        result.unwrap_or(vec![])
    }

    async fn get_root_task_batch(&self, take: i32, continuation_token: &str, sort_by: &str, descending: bool) -> (Vec<domain::models::TaskEntity>, String) {
        let sort = if descending { "DESC" } else { "ASC" };
        let skip = continuation_token.parse::<i32>().unwrap();
        let entities = 
            sqlx::query(format!("SELECT * FROM Tasks WHERE RootTaskId IS NULL ORDER BY {} {} LIMIT $1 OFFSET $2", sort_by, sort).as_str())
                .bind(take)
                .bind(skip)
                .map(|row: PgRow| {
                    convert::row_to_task_entity(&row)
                })
                .fetch_all(&self.pool)
                .await
                .unwrap();

        let skip = if {entities.len() as i32} < take { skip + entities.len() as i32 } else { skip + take };

        (entities, skip.to_string())
    }

    async fn search_tasks(&self, phrase: &str, take: i32, continuation_token: &str) -> (Vec<domain::models::TaskSearchEntity>, String) {
        let skip = continuation_token.parse::<i32>().unwrap();
        let entities = 
            sqlx::query("SELECT Id, Summary, Description FROM Tasks WHERE RootTaskId IS NULL AND (Summary ILIKE $1 OR Description ILIKE $2) ORDER BY CreateDate LIMIT $3 OFFSET $4")
                .bind(format!("%{}%", phrase))
                .bind(format!("%{}%", phrase))
                .bind(take)
                .bind(skip)
                .map(|row: PgRow| {
                    convert::row_to_task_search_entity(&row)
                })
                .fetch_all(&self.pool)
                .await
                .unwrap_or(vec![]);

        let skip = if {entities.len() as i32} < take { skip + entities.len() as i32 } else { skip + take };

        (entities, skip.to_string())
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
            .map(|row: PgRow| {
                Uuid::parse_str(row.get("val")).unwrap()
            })
            .fetch_all(&self.pool)
            .await;

        result.unwrap_or(vec![])
    }

    async fn update_task_root(&self, task_id: Uuid, new_root_id: Option<Uuid>) -> Result<(), app::errors::Error> {
        let affected = 
            sqlx::query("UPDATE Tasks SET RootTaskId = $1 WHERE Id = $2")
                .bind(new_root_id)
                .bind(task_id)
                .execute(&self.pool)
                .await
                .unwrap()
                .rows_affected();

        if affected > 0 { Ok(()) } else { Err(Error::EntityNotFound(task_id.to_string())) }
    }

    async fn update_task(&self, id: Uuid, summary: &str, description: Option<&str>, due_date: DateTime<Utc>, priority: domain::enums::TaskPriority, status: domain::enums::TaskStatus) -> Result<(), app::errors::Error> {
        let affected = 
            sqlx::query("UPDATE Tasks SET Summary = $1, Description = $2, DueDate = $3, Priority = $4, Status = $5 WHERE Id = $6")
                .bind(summary)
                .bind(description)
                .bind(due_date)
                .bind(convert::priority_to_i16(priority))
                .bind(convert::status_to_i16(status))
                .bind(id)
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
            sqlx::query("INSERT INTO Logs (Id, Action, TimestampMsec, EntityId, EntityType, Payload) VALUES ($1, $2, $3, $4, $5, $6)")
                .bind(entity.id)
                .bind(convert::action_to_i16(entity.action))
                .bind(entity.timestamp)
                .bind(entity.entity_id)
                .bind(entity.entity_type)
                .bind(entity.payload)
                .execute(&self.pool)
                .await;
    }

    async fn get_batch_by_entity_type(&self, entity_type: &str, continuation_token: &str, take: i32, descending: bool) -> (Vec<domain::models::LogEntity>, String) {
        let skip = continuation_token.parse::<i32>().unwrap();
        let sort = if descending { "DESC" } else { "ASC" };

        let entities = 
            sqlx::query(format!("SELECT * FROM Logs WHERE EntityType = $1 ORDER BY TimestampMsec {} LIMIT $3 OFFSET $4", sort).as_str())
                .bind(entity_type)
                .bind(sort)
                .bind(take)
                .bind(skip)
                .map(|row: PgRow| {
                    convert::row_to_log_entity(&row)
                })
                .fetch_all(&self.pool)
                .await
                .unwrap();

        let skip = if {entities.len() as i32} < take { skip + entities.len() as i32 } else { skip + take };

        (entities, skip.to_string())
    }

    async fn get_batch_by_entity(&self, entity_id: Uuid, continuation_token: &str, take: i32, descending: bool) -> (Vec<domain::models::LogEntity>, String) {
        let skip = continuation_token.parse::<i32>().unwrap();
        let sort = if descending { "DESC" } else { "ASC" };

        let entities = 
            sqlx::query("SELECT * FROM Logs WHERE EntityId = $1 ORDER BY TimestampMsec $2 LIMIT $3 OFFSET $4")
                .bind(entity_id)
                .bind(sort)
                .bind(take)
                .bind(skip)
                .map(|row: PgRow| {
                    convert::row_to_log_entity(&row)
                })
                .fetch_all(&self.pool)
                .await
                .unwrap();

        let skip = if {entities.len() as i32} < take { skip + entities.len() as i32 } else { skip + take };

        (entities, skip.to_string())
    }
}