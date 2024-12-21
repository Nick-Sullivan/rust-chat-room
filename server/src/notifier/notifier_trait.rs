use crate::domain::errors::LogicError;
use axum::async_trait;

#[async_trait]
pub trait INotifier: Send + Sync {
    async fn notify(&self, connection_id: &str, message: &str) -> Result<(), LogicError>;
}
