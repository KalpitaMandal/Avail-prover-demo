use actix_web::{get, http::StatusCode, post, web, Responder};
use serde_json::Value;
use std::{fs, str::FromStr};

use crate::{model, prover, response::response};

// Get generator status from the supervisord
#[get("/test")]
async fn test() -> impl Responder {
    response("The Avail prover is running!!", StatusCode::OK, None)
}

#[get("/benchmark")]
async fn benchmark() -> impl Responder {
    // Fetch config
    let config_path = "./app/config.json".to_string();
    let alt_config_path = "../app/config.json".to_string();
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

    log::info!("Printing benchmarks for the avail prover");

    let benchmark_proof_generation = prover::prove_authorization(config.private_key);

    match benchmark_proof_generation {
        Ok(benchmarks) => {
            let proving_time = benchmarks.proof_generation_time.to_string();
            return Ok(response(
                "Proof generated, the proof generation time returned is in milliseconds",
                StatusCode::OK,
                Some(Value::String(proving_time)),
            ));
        }
        Err(e) => {
            response(
                "There was an issue benchmarking the proof generation time.",
                StatusCode::INTERNAL_SERVER_ERROR,
                None,
            );
            return Err(e);
        }
    }
}

#[post("/generateProof")]
async fn generate_proof(payload: web::Json<model::ProverInputs>) -> impl Responder {
    // Fetch config
    let config_path = "./app/config.json".to_string();
    let alt_config_path = "../app/config.json".to_string();
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

    let private_input = payload.clone().private_input;
    let secrets = String::from_utf8(private_input).unwrap();
    log::info!("Secrets: {:?}", secrets);
    let value: Value = serde_json::from_str(&secrets).unwrap();
    log::info!("Secrets Value: {:?}", value);
    let private_inputs: prover::SecretInputs = serde_json::from_value(value).unwrap();
    log::info!("Secrets input format: {:?}", private_inputs);
    let prove_result;
    if private_inputs.private == "false".to_string() {
        log::info!("Generating proof for public market");
        prove_result = prover::prove_public(config.private_key, payload.0).await;
    } else {
        log::info!("Generating proof for private market");
        prove_result = prover::prove_private(config.private_key, payload.0).await;
    }

    match prove_result {
        Ok(prove) => {
            if prove.proof.is_some() {
                let public_inputs = prove.input.unwrap();
                let proof_bytes = prove.proof.unwrap();
                let signature = prove.signature.unwrap();
                let sig_bytes = ethers::types::Bytes::from_str(&signature).unwrap();
                let value = vec![
                    ethers::abi::Token::Bytes(public_inputs.to_vec()),
                    ethers::abi::Token::Bytes(proof_bytes.to_vec()),
                    ethers::abi::Token::Bytes(sig_bytes.to_vec()),
                ];
                let encoded = ethers::abi::encode(&value);
                let encoded_bytes: ethers::types::Bytes = encoded.into();
                return Ok(response(
                    "Proof generated",
                    StatusCode::OK,
                    Some(Value::String(encoded_bytes.to_string())),
                ));
            } else {
                let public_inputs = prove.input.unwrap();
                let signature = prove.signature.unwrap();
                let sig_bytes = ethers::types::Bytes::from_str(&signature).unwrap();
                let value = vec![
                    ethers::abi::Token::Bytes(public_inputs.to_vec()),
                    ethers::abi::Token::Bytes(sig_bytes.to_vec()),
                ];
                let encoded = ethers::abi::encode(&value);
                let encoded_bytes: ethers::types::Bytes = encoded.into();
                return Ok(response(
                    "Proof NOT generated",
                    StatusCode::BAD_REQUEST,
                    Some(Value::String(encoded_bytes.to_string())),
                ));
            }
        }
        Err(e) => {
            response(
                "There was an issue while generating the proof.",
                StatusCode::INTERNAL_SERVER_ERROR,
                None,
            );
            return Err(e);
        }
    }
}

#[post("/checkInput")]
async fn check_input(payload: web::Json<model::ProverInputs>) -> impl Responder {
    let private_input = payload.clone().private_input;
    let secrets = String::from_utf8(private_input).unwrap();
    let value: Value = serde_json::from_str(&secrets).unwrap();
    let private_inputs:Result<prover::SecretInputs, serde_json::Error> = serde_json::from_value(value);
    match private_inputs {
        Ok(private) => {
            let input_format_result;
            if private.private == "false".to_string() {
                log::info!("Checking input format for public market");
                input_format_result = prover::check_public(payload.0, false).await;
            } else {
                log::info!("Checking input format for private market");
                input_format_result = prover::check_private(payload.0, false).await;
            }

            match input_format_result {
                Ok(verified) => {
                    if verified.input_validity {
                        return Ok(response("Payload is valid", StatusCode::OK, None));
                    } else {
                        return Ok(response("Payload is NOT valid", StatusCode::OK, None));
                    }
                }
                Err(e) => {
                    response(
                        "There was an error validating public inputs",
                        StatusCode::NOT_IMPLEMENTED,
                        None,
                    );
                    return Err(e);
                }
            }
        }
        Err(_) => {
            response(
                "The private inputs are invalid",
                StatusCode::BAD_REQUEST,
                None,
            );
            return Err(model::InputError::InvalidInputs);
        }
    }
}

#[post("/checkInputWithSignature")]
async fn check_input_with_signature(payload: web::Json<model::ProverInputs>) -> impl Responder {
    let private_input = payload.clone().private_input;
    let secrets = String::from_utf8(private_input).unwrap();
    let value: Value = serde_json::from_str(&secrets).unwrap();
    let private_inputs:Result<prover::SecretInputs, serde_json::Error> = serde_json::from_value(value);
    match private_inputs {
        Ok(private) => {
            let input_format_result;
            if private.private == "false".to_string() {
                log::info!("Checking input format with signature for public market");
                input_format_result = prover::check_public(payload.0, true).await;
            } else {
                log::info!("Checking input format with signature for private market");
                input_format_result = prover::check_private(payload.0, true).await;
            }

            match input_format_result {
                Ok(verified) => {
                    let signature = verified.signature;
                    if verified.input_validity {
                        return Ok(response(
                            "Payload is valid",
                            StatusCode::OK,
                            Some(Value::String(signature.unwrap().to_string())),
                        ));
                    } else {
                        return Ok(response(
                            "Payload is NOT valid",
                            StatusCode::OK,
                            Some(Value::String(signature.unwrap().to_string())),
                        ));
                    }
                }
                Err(e) => {
                    response(
                        "There was an error validating public inputs",
                        StatusCode::NOT_IMPLEMENTED,
                        None,
                    );
                    return Err(e);
                }
            }
        }
        Err(_) => {
            response(
                "The private inputs are invalid",
                StatusCode::BAD_REQUEST,
                None,
            );
            return Err(model::InputError::InvalidInputs);
        }
    }
}

// Routes
pub fn routes(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api")
        .service(test)
        .service(benchmark)
        .service(generate_proof)
        .service(check_input)
        .service(check_input_with_signature);
    conf.service(scope);
}
