use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryEntry {
    pub id: i64,
    pub command: String,
    pub cwd: String,
    pub namespace: Option<String>,
    pub project: Option<String>,
    pub user: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionState {
    pub active_user: Option<String>,
    pub active_namespace: Option<String>,
    pub active_project: Option<String>,
}
