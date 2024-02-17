// Define a struct to represent a task
use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Task {
    title: String,
    task_url: String,
    task_url_type: TaskUrlType,
    task_url_web: Option<String>,
    callback_url: Option<String>,
    assigner: Option<String>,
    description: Option<String>,
    due_date: Option<DateTime<Utc>>,          // Changed to DateTime<Utc>
    expiration_date: Option<DateTime<Utc>>,   // Changed to DateTime<Utc>
    show_due_date: bool,
    past_due_duration_days: i32,
    created_at: DateTime<Utc>,                 // Changed to DateTime<Utc>
    completed_at: Option<DateTime<Utc>>,      // Changed to DateTime<Utc>
    owner_app: String,
    extra: Option<serde_json::Value>,
    date_available: Option<DateTime<Utc>>,   // Changed to DateTime<Utc>
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaskUrlType {
    Normal,
    Webview,
}
