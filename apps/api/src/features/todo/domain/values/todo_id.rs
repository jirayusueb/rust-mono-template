use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::shared::kernel::error::AppError;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct TodoId(Uuid);

impl TodoId {
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }
}

impl fmt::Display for TodoId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for TodoId {
    type Err = AppError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Uuid::parse_str(s)
            .map(Self)
            .map_err(|_| AppError::Validation("invalid id".into()))
    }
}

impl From<Uuid> for TodoId {
    fn from(u: Uuid) -> Self {
        Self(u)
    }
}

impl From<TodoId> for Uuid {
    fn from(id: TodoId) -> Self {
        id.0
    }
}

impl Default for TodoId {
    fn default() -> Self {
        Self::new()
    }
}
