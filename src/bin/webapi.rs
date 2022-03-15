use axum::{routing::get, Router};

use thrpg::webapi::owner;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(owner));

    let address = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));

    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .unwrap()
}
