use domain::{enums::{TaskAction, TaskPriority, TaskStatus}, models::{TaskEntity, TaskSearchEntity, LogEntity}};

use sqlx::{mysql::MySqlRow, Row};
use uuid::Uuid;

pub fn row_to_task_entity(row: &MySqlRow) -> TaskEntity {
    TaskEntity {
        id: Uuid::parse_str(row.get("Id")).unwrap(),
        root_task_id: Uuid::parse_str(row.get("RootTaskId")).map_or(None, |u| Some(u)),
        summary: row.get("Summary"),
        description: row.get("Description"),
        create_date: row.get("CreteDate"),
        due_date: row.get("DueDate"),
        priority: priority_from_str(row.get("Priority")),
        status: status_from_str(row.get("Status")),
    }
}

pub fn row_to_task_search_entity(row: &MySqlRow) -> TaskSearchEntity {
    TaskSearchEntity {
        id: Uuid::parse_str(row.get("Id")).unwrap(),
        summary: row.get("Summary"),
        description: row.get("Description"),
    }
}

pub fn row_to_log_entity(row: &MySqlRow) -> LogEntity {
    LogEntity { 
        id: Uuid::parse_str(row.get("Id")).unwrap(),
        action: action_from_str(row.get("Action")),
        timestamp: row.get("TimestampMsec"),
        entity_id: Uuid::parse_str(row.get("EntityId")).map_or(None, |u| Some(u)),
        entity_type: row.get("EntityType"),
        payload: row.get("Payload"),
    }
}

pub fn action_from_str(s: &str) -> TaskAction {
    match s {
        "Create" => TaskAction::Create,
        "Delete" => TaskAction::Delete,
        "Update" => TaskAction::Update,
        "RootChanged" => TaskAction::RootChanged,
        _ => panic!("Invalid value of TaskAction")
    }
}

pub fn action_to_str(a: TaskAction) -> String {
    match a {
        TaskAction::Create => String::from("Create"),
        TaskAction::Delete => String::from("Delete"),
        TaskAction::Update => String::from("Update"),
        TaskAction::RootChanged => String::from("RootChanged")
    }
}

pub fn priority_from_str(s: &str) -> TaskPriority {
    match s {
        "High" => TaskPriority::High,
        "Low" => TaskPriority::Low,
        "Normal" => TaskPriority::Normal,
        "Urgent" => TaskPriority::Urgent,
        _ => panic!("Invalid value of TaskPriority")
    }
}

pub fn priority_to_str(a: TaskPriority) -> String {
    match a {
        TaskPriority::High => String::from("High"),
        TaskPriority::Low => String::from("Low"),
        TaskPriority::Normal => String::from("Normal"),
        TaskPriority::Urgent => String::from("Urgent")
    }
}

pub fn status_from_str(s: &str) -> TaskStatus {
    match s {
        "Done" => TaskStatus::Done,
        "Ongoing" => TaskStatus::Ongoing,
        "Pending" => TaskStatus::Pending,
        "Reserved" => TaskStatus::Reserved,
        _ => panic!("Invalid value of TaskStatus")
    }
}

pub fn status_to_str(a: TaskStatus) -> String {
    match a {
        TaskStatus::Done => String::from("Done"),
        TaskStatus::Ongoing => String::from("Ongoing"),
        TaskStatus::Pending => String::from("Pending"),
        TaskStatus::Reserved => String::from("Reserved")
    }
}