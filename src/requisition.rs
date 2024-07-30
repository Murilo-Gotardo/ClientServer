use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Requisition {
    command: String,
    locate: String,
    value: Option<String>
}

impl Requisition {
    pub fn new(command: String, locate: String, value: Option<String>) -> Self {
        Self { command, locate, value }
    }

    pub fn command(&self) -> &str {
        &self.command
    }

    pub fn locate(&self) -> &str {
        &self.locate
    }

    pub fn value(&self) -> &Option<String> {
        &self.value
    }
}