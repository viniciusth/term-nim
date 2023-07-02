use axum::{routing::get, Router};
use local_ip_address::local_ip;

pub mod game;
pub mod ui;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    let local_ip = local_ip().unwrap();
    println!("{local_ip}");

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root));

    let url = format!("{}:3000", local_ip);

    axum::Server::bind(&url.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}
