use std::net::SocketAddr;

use axum::BoxError;
use chat_axum::router;

#[tokio::main]
async fn main() -> Result<(), BoxError> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3002));
    let app = router();

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;
    Ok(())
}
