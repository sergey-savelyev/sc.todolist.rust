use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct Pagination {
    take: i32,
    skip: i32,
    sort_by: Option<String>,
    descending_sort: Option<bool>,
    descending: Option<bool> // backward compatibility with C# project
}

impl Pagination {
    pub fn take(&self) -> i32 { self.take }
    pub fn skip(&self) -> i32 { self.skip }
    pub fn sort_by(&self) -> Option<&str> { 
        if let Some(s) = self.sort_by.as_ref() {
            return Some(s);
        }

        None
    }
    pub fn descending_sort(&self) -> Option<bool> { self.descending_sort }
    pub fn descending(&self) -> Option<bool> { self.descending }
}

#[derive(Debug, Serialize)]
pub struct BatchResponse<T> {
    entities: Vec<T>,
    continuation_token: String
}

impl<T> BatchResponse<T> {
    // WARNING: intentionally consumes ownerships, since it's a final destination
    pub fn new(entities: Vec<T>, continuation_token: String) -> BatchResponse<T> {
        BatchResponse { entities, continuation_token }
    }
}

#[derive(Serialize)]
pub struct CreateTaskResponse {
    task_id: Uuid
}

impl CreateTaskResponse {
    pub fn new(task_id: Uuid) -> CreateTaskResponse { CreateTaskResponse { task_id } }
}

#[derive(Deserialize)]
pub struct TaskRootChangeRequest {
    root_id: Option<Uuid>
}

impl TaskRootChangeRequest {
    pub fn root_id(&self) -> Option<Uuid> { self.root_id }
}