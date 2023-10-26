use app::{tasks::TaskService, logs::LogService};
use db::{LogStorage, TaskStorage};
use sqlx::{postgres::PgPoolOptions, PgPool};

use std::{time::Duration, sync::Arc};

pub mod db;
pub mod convert;

pub struct ServiceProvider {
    pool: PgPool
}

impl ServiceProvider {
    pub fn new(connection_string: &str) -> ServiceProvider {
        ServiceProvider { 
            pool: PgPoolOptions::new()
                    .max_connections(10)
                    .acquire_timeout(Duration::from_secs(3))
                    .connect_lazy(connection_string)
                    .expect("can't connect to database")
        }
    }

    pub fn task_service(&self) -> Arc<TaskService> {
        Arc::new(TaskService::new(Arc::new(TaskStorage::new(self.pool.clone())), Arc::new(LogService::new(Arc::new(LogStorage::new(self.pool.clone()))))))
    }

    pub fn log_service(&self) -> Arc<LogService> {
        Arc::new(LogService::new(Arc::new(LogStorage::new(self.pool.clone()))))
    }
}