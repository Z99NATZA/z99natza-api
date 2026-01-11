use crate::modules::chat::domain::chat::Chat;

pub fn build_context(history: &[Chat]) -> String {
    history
        .iter()
        .map(|chat| format!("{}: {}", chat.sender, chat.message))
        .collect::<Vec<_>>()
        .join("\n")
}