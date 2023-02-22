use std::{collections::HashMap, convert::Infallible, sync::Arc, time::Duration};

use axum::{
    body::Bytes,
    error_handling::HandleErrorLayer,
    extract::{DefaultBodyLimit, Path, State},
    handler::Handler,
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get},
    BoxError, Router,
};
use hyper::{Body, Request};
use tokio::sync::RwLock;
use tower::{timeout::TimeoutLayer, ServiceBuilder};
use tower_http::{
    auth::RequireAuthorizationLayer, limit::RequestBodyLimitLayer, trace::TraceLayer,
};

/// Custom type for a shared state
pub type SharedState = Arc<RwLock<AppState>>;

use tracing::{event, instrument, Level};

use tracing_subscriber::FmtSubscriber;

mod log;

#[derive(Default, Debug)]
pub struct AppState(HashMap<String, Bytes>);

pub fn router(state: &SharedState) -> Router {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::DEBUG)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("Setting a default Subscriber failed");

    let kv_set_service = ServiceBuilder::new()
        .layer(HandleErrorLayer::new(handle_error))
        .layer(DefaultBodyLimit::disable())
        .layer(RequestBodyLimitLayer::new(1024 * 8000))
        .layer(TimeoutLayer::new(Duration::from_secs(4)))
        .service(kv_store_set.with_state(Arc::clone(state)));

    Router::new()
        .route(
            "/kv/:key",
            get(kv_store_get)
                .post_service(kv_set_service)
                .with_state(Arc::clone(state)),
        )
        .nest("/admin", admin_routes(state))
        .layer(TraceLayer::new_for_http())
}

fn admin_routes(state: &SharedState) -> Router {
    async fn remove_key(
        Path(key): Path<String>,
        State(state): State<SharedState>,
    ) -> Result<(), Infallible> {
        state.write().await.0.remove(&key);
        Ok(())
    }

    async fn delete_all_keys(
        State(state): State<SharedState>,
        request: Request<Body>,
    ) -> Result<(), Infallible> {
        println!("{:?}", request.extensions());
        state.write().await.0.clear();
        Ok(())
    }

    Router::new()
        .route(
            "/keys",
            delete(delete_all_keys).with_state(Arc::clone(state)),
        )
        .route(
            "/keys/:key",
            delete(remove_key).with_state(Arc::clone(state)),
        )
        .layer(RequireAuthorizationLayer::custom(
            |req: &mut Request<Body>| {
                req.extensions_mut().insert("admin");
                println!("{:?}", req.headers());
                Ok(())
            },
        ))
}

#[instrument(level = "debug")]
async fn kv_store_get(
    Path(key): Path<String>,
    State(state): State<SharedState>,
) -> Result<Bytes, StatusCode> {
    let db = state.read().await;

    tokio::time::sleep(Duration::from_secs(3)).await;

    if let Some(val) = db.0.get(&key) {
        event!(Level::DEBUG, "Found");
        Ok(val.to_owned())
    } else {
        event!(Level::DEBUG, "Not Found");
        Err(StatusCode::NOT_FOUND)
    }
}

async fn kv_store_set(
    Path(key): Path<String>,
    State(state): State<SharedState>,
    bytes: Bytes,
) -> Result<(), StatusCode> {
    state.write().await.0.insert(key, bytes);
    Ok(())
}

async fn handle_error(error: BoxError) -> impl IntoResponse {
    if error.is::<tower::timeout::error::Elapsed>() {
        return (
            StatusCode::REQUEST_TIMEOUT,
            String::from("request timed out"),
        );
    }

    (
        StatusCode::INTERNAL_SERVER_ERROR,
        String::from(format!("Unhandled internal error: {}", error)),
    )
}

#[cfg(test)]
mod tests {
    use axum::{
        body::Bytes,
        http::{Request, StatusCode},
    };
    use hyper::Body;
    use tower::Service;

    use crate::{router, SharedState};

    #[tokio::test]
    async fn basic_kv_store_post_test() {
        let state = SharedState::default();
        let mut app = router(&state);

        let request = Request::builder()
            .uri("/kv/test")
            .method("POST")
            .body("Hello World".into())
            .unwrap();

        let response = app.call(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let db = state.read().await;
        let result = db.0.get("test").unwrap();
        assert_eq!(&result[..], b"Hello World");
    }

    #[tokio::test]
    async fn basic_kv_store_get_404() {
        let state = SharedState::default();
        let mut app = router(&state);

        let request = Request::builder()
            .uri("/kv/test")
            .method("GET")
            .body(Body::empty())
            .unwrap();

        let response = app.call(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn basic_kv_store_get() {
        let state = SharedState::default();
        state
            .write()
            .await
            .0
            .insert("test".to_string(), Bytes::from_static(b"Hello World"));
        let mut app = router(&state);

        let request = Request::builder()
            .uri("/kv/test")
            .method("GET")
            .body(Body::empty())
            .unwrap();

        let response = app.call(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        assert_eq!(&body[..], "Hello World".as_bytes());
    }
}
