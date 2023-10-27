use app::{tasks::TaskService, logs::LogService};
use db::{LogStorage, TaskStorage };
use sqlx::postgres::PgPoolOptions;

use std::{time::Duration, sync::Arc};

pub mod db;
pub mod convert;

pub struct ServiceProvider {
    task_service: Arc<TaskService>,
    log_service: Arc<LogService>
}

impl ServiceProvider {
    pub fn new(connection_string: &str) -> ServiceProvider {
        let pool = PgPoolOptions::new()
            .max_connections(10)
            .acquire_timeout(Duration::from_secs(3))
            .connect_lazy(connection_string)
            .expect("can't connect to database");

        // Arc<T> is a thread-safe reference count pointer, actually when clone() called it just passing the same pointer, but increasing ref count
        // Exactly what we need here
        let log_ervice_ptr: Arc<LogService> = Arc::new(LogService::new(Arc::new(LogStorage::new(pool.clone()))));

        ServiceProvider { 
            task_service: Arc::new(TaskService::new(Arc::new(TaskStorage::new(pool.clone())), Arc::clone(&log_ervice_ptr))),
            log_service: log_ervice_ptr
        }
    }

    pub fn task_service(&self) -> Arc<TaskService> {
        self.task_service.clone()
    }

    pub fn log_service(&self) -> Arc<LogService> {
        self.log_service.clone()
    }
}