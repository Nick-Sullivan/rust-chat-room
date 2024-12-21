use std::sync::Arc;

use crate::{domain::errors::LogicError, notifier::notifier_trait::INotifier};

pub async fn on_message(
    connection_id: &str,
    notifier: &Arc<dyn INotifier>,
) -> Result<(), LogicError> {
    tracing::info!("on_disconnect!");
    notifier.notify(connection_id, "Hi").await?;
    Ok(())
}
