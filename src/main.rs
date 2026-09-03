mod logic;
mod schema;
mod server;

use crate::{logic::Market, server::create_app};
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let store = Market::new();
    let app = create_app(Arc::new(store));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("failed to bind tcp listener");

    println!("Server running on http://localhost:3000");

    axum::serve(listener, app)
        .await
        .expect("failed to start server")
}
