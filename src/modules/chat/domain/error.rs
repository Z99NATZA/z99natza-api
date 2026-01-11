#[derive(Debug)]
pub enum ChatError {
    NotFound,
    InvalidChatId,
    PersistenceFailure,
}