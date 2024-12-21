use crate::domain::errors::LogicError;

pub async fn on_disconnect(_connection_id: &str) -> Result<(), LogicError> {
    tracing::info!("on_disconnect!");
    // notifier.notify("connection_id", "connecting").await?;
    Ok(())
}
