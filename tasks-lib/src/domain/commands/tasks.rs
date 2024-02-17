use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
struct CreateTaskCommand {
    profile_id: String,
    org_id: i32,
    title: String,
    task_url: String,
    task_url_type: TaskUrlType,
    task_url_web: Option<String>,
    callback_url: Option<String>,
    assigner: Option<String>,
    description: Option<String>,
    due_date: Option<DateTime<Utc>>,
    show_due_date: bool,
    past_due_duration_days: i32,
    expiration_date: Option<DateTime<Utc>>,
    extra: HashMap<String, String>, // Change the value type accordingly
    date_available: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
enum TaskUrlType {
    NORMAL,
    WEBVIEW,
}

impl Default for CreateTaskCommand {
    fn default() -> Self {
        Self {
            profile_id: String::default(),
            org_id: 0,
            title: String::default(),
            task_url: String::default(),
            task_url_type: TaskUrlType::NORMAL,
            task_url_web: None,
            callback_url: None,
            assigner: None,
            description: None,
            due_date: Some(Utc::now()), // You might want to adjust this default
            show_due_date: false,
            past_due_duration_days: 7,
            expiration_date: None,
            extra: HashMap::default(),
            date_available: None,
        }
    }
}
