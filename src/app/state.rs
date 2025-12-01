use std::env;

use crate::ai::provider::{AiProvider, ai_type::AiType};

pub struct AppState {
    pub ai: AiProvider
}

impl AppState {
    pub fn new() -> Self {
        Self { ai: AiType::config().build() }
    }
}


