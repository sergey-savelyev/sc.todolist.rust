use uuid::Uuid;

pub enum Error {
    EntityNotFound(String),
    InvalidRootBinding(String)
}

impl Error {
    pub fn not_found(id: Uuid) -> Self {
        Error::EntityNotFound(format!("Entity with id {} cannot be found", id))
    }

    pub fn invalid_root_binding(message: &str) -> Self {
        Error::InvalidRootBinding(message.to_string())
    }
}