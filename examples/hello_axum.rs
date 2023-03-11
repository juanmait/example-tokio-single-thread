//! Run with
//!
//! ```not_rust
//! cargo run --example hello_axum
//! 
//! curl http://localhost:3000
//! ```

use axum::{response::Html, routing::get, Router};
use std::net::SocketAddr;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    // build our application with a route
    let app = Router::new().route("/", get(handler));

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler() -> Html<&'static str> {
    println!("Attending new request...");
    Html("<h1>Hello, World!</h1>")
}
