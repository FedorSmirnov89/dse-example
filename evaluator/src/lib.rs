use std::sync::Arc;

use axum::{response::Response, Extension, Json};
use evaluation::evaluate_config;
use types::{read_json_input, ConfigJson, EvaluationResult};

pub use types::DelayConfiguration;

mod error;
mod evaluation;
mod types;

#[axum::debug_handler]
pub async fn serve_eval_request(
    Extension(delay_config): Extension<Arc<DelayConfiguration>>,
    Json(config_json): Json<ConfigJson>,
) -> Result<Json<EvaluationResult>, Response> {
    println!("received config json: {config_json:?}");
    let config = read_json_input(config_json)?;
    let eval_result = evaluate_config(config);
    delay_config.simulate_delay().await?;
    println!("input evaluated: {eval_result:?}");
    Ok(Json(eval_result))
}
