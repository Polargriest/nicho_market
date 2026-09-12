mod logic;
mod schema;
mod server;

use crate::server::create_app;
use std::{
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use crate::logic::market::Market;

const DATABASE_PATH: &str = "database.json";

fn setup() -> Market {
    Market::load_market(DATABASE_PATH)
}

#[tokio::main]
async fn main() {
    let market = setup();
    let app = create_app(Arc::new(Mutex::new(market)));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("failed to bind tcp listener");

    println!("Server running on http://localhost:3000");

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}
