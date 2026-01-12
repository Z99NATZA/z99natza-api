use crate::{app::AppError, modules::ai_chat::domain::ChatError};

pub fn map_chat_error(err: ChatError) -> AppError {
    match err {
        ChatError::NotFound =>
            AppError::NotFound,

        ChatError::InvalidChatId =>
            AppError::BadRequest("Invalid chat id".into()),

        ChatError::PersistenceFailure =>
            AppError::GenericError,
    }
}
