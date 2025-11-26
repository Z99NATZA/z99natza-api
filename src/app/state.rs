use crate::ai::provider::{AiProvider, AiProviderKind};
use std::env;

pub struct AppState {
    pub ai: AiProvider,
}

impl AppState {
    pub fn new() -> Self {
        let kind = match env::var("AI_PROVIDER").as_deref() {
            Ok("openai") => AiProviderKind::OpenAI,
            _ => AiProviderKind::OpenAI,
        };

        Self {
            ai: kind.build(),
        }
    }
}
