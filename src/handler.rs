use actix_web::{
    get, post, web, HttpResponse
};
use std::{fs, time::Instant};
use aleo_rust::{
    AleoAPIClient,
    snarkvm_types::Testnet3
  };
use serde_json::json;

use crate::{model, prover};

// Get generator status from the supervisord
#[get("/test")]
async fn test() -> Result<HttpResponse, model::InputError> {
    let total_now = Instant::now();
    // Create a client that interacts with the testnet3 program
    let api_client = AleoAPIClient::<Testnet3>::testnet3();

    // Check if correct program is getting fetched 
    let _credits = api_client.get_program("credits.aleo").unwrap();
    let total_time = total_now.elapsed();

    let response = json!({
        "Time taken for fetching deployed program": total_time.as_millis().to_string() + "ms"
    });
    return Ok(HttpResponse::Ok().body(serde_json::to_string(&response).unwrap()));
}

#[get("/benchmark")]
async fn benchmark(
    payload: web::Json<model::ProverInputs>
) -> Result<HttpResponse, model::InputError> {
    // Fetch config
    let config_path = "./config.json".to_string();
    let alt_config_path = "../config.json".to_string();
    let file_content =
        fs::read_to_string(config_path).or_else(|_| fs::read_to_string(alt_config_path));
    if file_content.is_err() {
        log::error!("{:#?}", file_content.err());
        return Err(model::InputError::FileNotFound);
    }

    let config: model::ProverConfig = match serde_json::from_str(&file_content.unwrap()) {
        Ok(data) => data,
        Err(err) => {
            log::error!("{}", err);
            return Err(model::InputError::BadConfigData);
        }
    };

    let prove_result = prover::prove_authorization(config.private_key, payload.0);

    match prove_result {
        Ok(result) => {
            return Ok(result);
        }
        Err(e) => {
            return Err(e);
        }
    }
}


// To execute deployed programs
#[post("/execute")]
async fn execute(
    payload: web::Json<model::ProverInputs>
) -> Result<HttpResponse, model::InputError> {
    // Fetch config
    let config_path = "./config.json".to_string();
    let alt_config_path = "../config.json".to_string();
    let file_content =
        fs::read_to_string(config_path).or_else(|_| fs::read_to_string(alt_config_path));
    if file_content.is_err() {
        log::error!("{:#?}", file_content.err());
        return Err(model::InputError::FileNotFound);
    }

    let config: model::ProverConfig = match serde_json::from_str(&file_content.unwrap()) {
        Ok(data) => data,
        Err(err) => {
            log::error!("{}", err);
            return Err(model::InputError::BadConfigData);
        }
    };
    let execute_result = prover::execute_credits_transfer_public(
        config,
        payload.0
    );

    match execute_result {
        Ok(result) => {
            return Ok(result);
        }
        Err(e) => {
            return Err(e);
        }
    }
}

#[post("/generateProof")]
async fn generate_proof(
    payload: web::Json<model::ProverInputs>
) -> Result<HttpResponse, model::InputError> {
    // Fetch config
    let config_path = "./config.json".to_string();
    let alt_config_path = "../config.json".to_string();
    let file_content =
        fs::read_to_string(config_path).or_else(|_| fs::read_to_string(alt_config_path));
    if file_content.is_err() {
        log::error!("{:#?}", file_content.err());
        return Err(model::InputError::FileNotFound);
    }

    let config: model::ProverConfig = match serde_json::from_str(&file_content.unwrap()) {
        Ok(data) => data,
        Err(err) => {
            log::error!("{}", err);
            return Err(model::InputError::BadConfigData);
        }
    };

    log::info!(
        "Request received by the avail prover for ask ID : {}",
        payload.0.ask_id
    );

    let prove_result = prover::prove_multi(config.private_key, payload.0).await;

    match prove_result {
        Ok(result) => {
            return Ok(result);
        }
        Err(e) => {
            return Err(e);
        }
    }
}

// Routes
pub fn routes(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api")
        .service(test)
        .service(benchmark)
        .service(generate_proof)
        .service(execute);
    conf.service(scope);
}
