use key_value_store::{router, SharedState};
use std::net::SocketAddr;

type BoxError = Box<dyn std::error::Error>;

#[tokio::main]
async fn main() -> Result<(), BoxError> {
    let state = SharedState::default();
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let app = router(&state);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
