use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Content {
    pub key: String,
    pub value: String,
    pub exp: bool,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub exp_at: DateTime<Utc>,
}

impl Content {
    pub fn new(key: String, value: String, ttl: u64) -> Self {
        let now = Utc::now();
        let exp_date = now + chrono::Duration::seconds(ttl as i64);
        Content {
            key,
            value,
            exp: ttl > 0,
            created_at: now,
            exp_at: exp_date,
        }
    }
}
