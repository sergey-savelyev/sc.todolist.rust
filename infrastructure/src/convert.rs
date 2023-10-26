use domain::{enums::{TaskAction, TaskPriority, TaskStatus}, models::{TaskEntity, TaskSearchEntity, LogEntity}};

use sqlx::{postgres::PgRow, Row};
use uuid::Uuid;

pub fn row_to_task_entity(row: &PgRow) -> TaskEntity {
    TaskEntity {
        id: row.get("id"),
        root_task_id: row.get("roottaskid"),
        summary: row.get("summary"),
        description: row.get("description"),
        create_date: row.get("createdate"),
        due_date: row.get("duedate"),
        priority: priority_from_i16(row.get("priority")),
        status: status_from_i16(row.get("status")),
    }
}

pub fn row_to_task_search_entity(row: &PgRow) -> TaskSearchEntity {
    TaskSearchEntity {
        id: Uuid::parse_str(row.get("id")).unwrap(),
        summary: row.get("summary"),
        description: row.get("description"),
    }
}

pub fn row_to_log_entity(row: &PgRow) -> LogEntity {
    LogEntity {
        id: Uuid::parse_str(row.get("id")).unwrap(),
        action: action_from_i16(row.get("action")),
        timestamp: row.get("timestampmsec"),
        entity_id: Uuid::parse_str(row.get("entityid")).map_or(None, |u| Some(u)),
        entity_type: row.get("entitytype"),
        payload: row.get("payload"),
    }
}

pub fn action_from_i16(u: i16) -> TaskAction {
    match u {
        0 => TaskAction::Create,
        1 => TaskAction::Delete,
        2 => TaskAction::Update,
        3 => TaskAction::RootChanged,
        _ => panic!("Invalid value of TaskAction")
    }
}

pub fn action_to_i16(a: TaskAction) -> i16 {
    match a {
        TaskAction::Create => 0,
        TaskAction::Delete => 1,
        TaskAction::Update => 2,
        TaskAction::RootChanged => 3
    }
}

pub fn priority_from_i16(u: i16) -> TaskPriority {
    match u {
        0 => TaskPriority::Low,
        1 => TaskPriority::Normal,
        2 => TaskPriority::High,
        3 => TaskPriority::Urgent,
        _ => panic!("Invalid value of TaskPriority")
    }
}

pub fn priority_to_i16(a: TaskPriority) -> i16 {
    match a {
        TaskPriority::Low => 0,
        TaskPriority::Normal => 1,
        TaskPriority::High => 2,
        TaskPriority::Urgent => 3
    }
}

pub fn status_from_i16(u: i16) -> TaskStatus {
    match u {
        0 => TaskStatus::Reserved,
        1 => TaskStatus::Ongoing,
        2 => TaskStatus::Done,
        3 => TaskStatus::Pending,
        _ => panic!("Invalid value of TaskStatus")
    }
}

pub fn status_to_i16(a: TaskStatus) -> i16 {
    match a {
        TaskStatus::Reserved => 0,
        TaskStatus::Ongoing => 1,
        TaskStatus::Done => 2,
        TaskStatus::Pending => 3,
    }
}