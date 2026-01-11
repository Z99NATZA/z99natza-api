use std::fmt;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ChatId(String);

impl fmt::Display for ChatId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}