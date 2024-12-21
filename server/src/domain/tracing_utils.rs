use axum::body::Body;
use hyper::Request;
use tracing_subscriber;

pub fn init_tracing() {
    tracing_subscriber::fmt()
        .json()
        .with_max_level(tracing::Level::INFO)
        .init();
}

pub fn trace_on_request(request: &Request<Body>, _: &tracing::Span) {
    let route_key = request.uri().path();
    // let context = request.request_context();

    tracing::info!(
        method = %request.method(),
        uri = %request.uri(),
        headers = ?request.headers(),
        route_key = %route_key,
        // context = ?context,
        message = "begin request!"
    )
}
