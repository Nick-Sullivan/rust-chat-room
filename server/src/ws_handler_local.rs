mod domain;
mod notifier;
mod service;

use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::extract::State;
use axum::response::Response;
use axum::{routing::any, Router};
use domain::errors::LogicError;
use domain::tracing_utils;
use futures_util::stream::StreamExt;
use notifier::notifier_local::NotifierLocal;
use notifier::notifier_trait::INotifier;
use std::error::Error;
use std::sync::Arc;
use tower_http::trace::TraceLayer;

const REQUEST_ID: &str = "123";

struct AppState {
    notifier: Arc<dyn INotifier>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    tracing_utils::init_tracing();
    let state = make_state().await;
    let trace_layer = TraceLayer::new_for_http().on_request(tracing_utils::trace_on_request);
    let app = Router::new()
        .route("/", any(initialise_connection))
        .with_state(state)
        .layer(trace_layer);
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}

async fn make_state() -> Arc<AppState> {
    Arc::new(AppState {
        notifier: Arc::new(NotifierLocal::new().await),
    })
}

#[axum::debug_handler]
async fn initialise_connection(
    State(state): State<Arc<AppState>>,
    ws: WebSocketUpgrade,
) -> Result<Response, LogicError> {
    tracing::info!("Upgrading to WebSocket");
    let notifier = state.notifier.clone();
    service::on_connect::on_connect(REQUEST_ID).await?;
    let response = ws.on_upgrade(|socket| async {
        if let Err(e) = handle_socket(socket, notifier).await {
            tracing::error!("Error handling socket: {:?}", e);
        }
    });
    Ok(response)
}

async fn handle_socket(
    mut socket: WebSocket,
    notifier: Arc<dyn INotifier>,
) -> Result<(), LogicError> {
    while let Some(Ok(msg)) = socket.next().await {
        if let Message::Text(text) = msg {
            service::on_message::on_message(REQUEST_ID, &notifier).await?;
            let response = Message::Text(text);
            if socket.send(response).await.is_err() {
                break;
            }
        }
    }
    service::on_disconnect::on_disconnect(REQUEST_ID).await?;
    Ok(())
}
