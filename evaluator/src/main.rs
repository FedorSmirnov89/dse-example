use axum::{routing::post, Extension, Router};
use evaluator::{serve_eval_request, DelayConfiguration};
use std::{net::SocketAddr, sync::Arc};

const FILE_DELAY_CONFIG: &str = "delay_config.json";

#[tokio::main]
async fn main() {
    let delay_config = read_delay_config();

    let app = Router::new()
        .route("/evaluate", post(serve_eval_request))
        .layer(Extension(Arc::new(delay_config)));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🔧 Server listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

fn read_delay_config() -> DelayConfiguration {
    let bytes = std::fs::read(FILE_DELAY_CONFIG).expect("failed to read delay config file");
    serde_json::from_slice(&bytes).expect("failed to deser delay config")
}
