use crate::domain::errors::LogicError;

pub async fn on_connect(_connection_id: &str) -> Result<(), LogicError> {
    tracing::info!("on_connect!");
    // notifier.notify(connection_id, "Connected").await?;
    Ok(())
}
