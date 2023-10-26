use app::{tasks::TaskService, logs::LogService};
use db::{LogStorage, TaskStorage};
use sqlx::mysql::MySqlPoolOptions;

use std::{time::Duration, rc::Rc};

pub mod db;
pub mod convert;

pub struct ServiceProvider(Box<TaskService>, Rc<LogService>);

pub async fn create_service_provider(connection_string: &str) -> ServiceProvider {
    let pool = MySqlPoolOptions::new()
        .max_connections(10)
        .acquire_timeout(Duration::from_secs(3))
        .connect(connection_string)
        .await
        .expect("can't connect to database");

    let log_repo = Box::new(LogStorage::new(pool.clone()));
    let task_repo = Box::new(TaskStorage::new(pool.clone()));

    let log_service = Rc::new(LogService::new(log_repo));
    let task_service = Box::new(TaskService::new(task_repo, Rc::clone(&log_service)));

    ServiceProvider(task_service, log_service)
}