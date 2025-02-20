use std::collections::HashMap;

use crate::{content::Content, error::DBError};

pub struct Db {
    store: tokio::sync::Mutex<HashMap<String, Content>>,
}

impl Db {
    pub fn new() -> Self {
        Self {
            store: tokio::sync::Mutex::new(HashMap::new()),
        }
    }

    pub async fn get(&self, key: &str) -> Result<Option<Content>, DBError> {
        let mut store = self.store.lock().await;
        match store.get(key) {
            Some(res) => {
                if res.exp_at < chrono::Utc::now() {
                    store.remove(key);
                    Err(DBError::Expired)
                } else {
                    Ok(Some(res.clone()))
                }
            }
            None => Err(DBError::NotFound),
        }
    }

    pub async fn set(&self, key: &str, value: String, ttl: u64) -> Result<Content, DBError> {
        let val = Content::new(key.to_string(), value, ttl);
        self.store
            .lock()
            .await
            .entry(key.to_string())
            .and_modify(|e| *e = val.clone())
            .or_insert(val.clone());

        Ok(val)
    }

    pub async fn del(&self, key: &str) -> Result<(), DBError> {
        let mut store = self.store.lock().await;
        match store.remove(key) {
            Some(_) => Ok(()),
            None => Err(DBError::NotFound),
        }
    }
}
