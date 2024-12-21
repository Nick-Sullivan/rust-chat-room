#![allow(dead_code)]
use crate::domain::errors::LogicError;
use crate::notifier::notifier_trait::INotifier;
use axum::async_trait;
use std::collections::HashMap;
use std::sync::RwLock;

pub struct NotifierLocal {
    pub log: RwLock<HashMap<String, Vec<String>>>,
}

impl NotifierLocal {
    pub async fn new() -> Self {
        let log = RwLock::new(HashMap::new());
        NotifierLocal { log }
    }
}

#[async_trait]
impl INotifier for NotifierLocal {
    async fn notify(&self, connection_id: &str, message: &str) -> Result<(), LogicError> {
        let mut hash_map = self.log.write().unwrap();
        match hash_map.get_mut(connection_id) {
            Some(log) => log.push(message.to_string()),
            None => {
                hash_map.insert(connection_id.to_string(), vec![message.to_string()]);
            }
        }
        Ok(())
    }
}
