use crate::ai::provider::{AiProvider, AiProviderKind};
use std::env;

pub struct AppState {
    pub ai: AiProvider,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            ai: AiProviderKind::from_env().build(),
        }
    }
}
