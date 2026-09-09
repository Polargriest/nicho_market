mod logic;
mod schema;
mod server;

use crate::server::create_app;
use std::{
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use crate::logic::market::Market;

fn setup() -> Market {
    let mut store = Market::new();

    store.add_ticker(0, "Joge", "La estamos rompiendo");
    store.add_user("Edy Figueroa");
    store.add_user("Quintero Nose");
    store.set_admin_perms(0, true).unwrap();
    store.set_money_for_user(0, 100_000).unwrap();
    store.set_money_for_user(1, 100_000).unwrap();

    store
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
