#![allow(dead_code)]
use crate::domain::errors::LogicError;
use crate::notifier::notifier_trait::INotifier;
use axum::async_trait;
use std::collections::HashMap;
use std::sync::RwLock;

pub struct NotifierFake {
    pub log: RwLock<HashMap<String, Vec<String>>>,
}

impl NotifierFake {
    pub async fn new() -> Self {
        let log = RwLock::new(HashMap::new());
        NotifierFake { log }
    }

    pub fn get_log(&self, connection_id: &str) -> Vec<String> {
        let hash_map = self.log.read().unwrap();
        let default: Vec<String> = vec![];
        hash_map.get(connection_id).unwrap_or(&default).clone()
    }
}

#[async_trait]
impl INotifier for NotifierFake {
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
