pub enum TaskPriority {
    Low,
    Normal,
    High,
    Urgent,
}

pub enum TaskStatus {
    Reserved,
    Ongoing,
    Done,
    Pending,
}

pub enum TaskAction {
    Create,
    Delete,
    Update,
    RootChanged,
}