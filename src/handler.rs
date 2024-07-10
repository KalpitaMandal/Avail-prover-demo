use actix_web::{get, http::StatusCode, post, web, Responder};
use aleo_rust::Testnet3;
use ethers::signers::{LocalWallet, Signer};
use serde_json::{Error, Value};
use snarkvm_synthesizer::Authorization;
use std::{fs, str::FromStr};

use crate::{model, prover, response::response};

// Get generator status from the supervisord
#[get("/test")]
async fn test() -> impl Responder {
    response("The Avail prover is running!!", StatusCode::OK, None)
}

#[get("/benchmark")]
async fn benchmark() -> impl Responder {
    // Fetch authorization
    let auth_path = "./app/auth_test.txt".to_string();
    let alt_auth_path = "../app/auth_test.txt".to_string();
    let file_content = fs::read_to_string(auth_path).or_else(|_| fs::read_to_string(alt_auth_path));

    if file_content.is_err() {
        log::error!("{:#?}", file_content.err());
        return Err(model::InputError::FileNotFound);
    }

    let auth_value: Value = serde_json::from_str(&file_content.unwrap()).unwrap();
    let authorization_structure: Result<Authorization<Testnet3>, Error> =
        serde_json::from_value(auth_value);

    if authorization_structure.is_err() {
        log::error!("{:#?}", authorization_structure.err());
        return Err(model::InputError::InvalidInputs);
    }

    log::info!("Printing benchmarks for the avail prover");
    let benchmark_proof_generation = prover::prove_authorization(authorization_structure.unwrap());

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
async fn generate_proof(payload: web::Json<model::ProveAuthInputs>) -> impl Responder {
    log::info!(
        "Request received by the avail prover for ask ID : {}",
        payload.0.ask_id
    );

    let prove_result = prover::prove_auth(payload.0).await;

    match prove_result {
        Ok(prove) => {
            if prove.execution.is_some() && prove.signature.is_some() {
                let public_inputs = prove.input.unwrap();
                let proof_bytes = prove.execution.unwrap();
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
            } else if prove.execution.is_none() && prove.signature.is_some() {
                let signature = prove.signature.unwrap();
                return Ok(response(
                    "Invalid inputs received, signature generated",
                    StatusCode::BAD_REQUEST,
                    Some(Value::String(signature)),
                ));
            } else {
                return Ok(response(
                    "There was an issue while generating the proof.",
                    StatusCode::INTERNAL_SERVER_ERROR,
                    None,
                ));
            }
        }
        Err(e) => {
            return Err(e);
        }
    }
}

#[post("/checkInput")]
async fn check_input_handler(payload: web::Json<model::ProveAuthInputs>) -> impl Responder {
    let private_input = payload.clone().private_input;
    let secrets = String::from_utf8(private_input).unwrap();
    let auth_value: Value = serde_json::from_str(&secrets).unwrap();
    let authorization_structure: Result<Authorization<Testnet3>, Error> =
        serde_json::from_value(auth_value);

    match authorization_structure {
        Ok(auth) => {
            let is_auth_empty = auth.is_empty();
            if is_auth_empty {
                return Ok(response("Payload is NOT valid", StatusCode::OK, None));
            } else {
                return Ok(response("Payload is valid", StatusCode::OK, None));
            }
        }
        Err(_) => {
            response(
                "The authorization input structure is invalid",
                StatusCode::BAD_REQUEST,
                None,
            );
            return Err(model::InputError::InvalidInputs);
        }
    }
}

#[post("/checkInputWithSignature")]
async fn check_input_with_signature(payload: web::Json<model::ProveAuthInputs>) -> impl Responder {
    let private_input = payload.clone().private_input;
    let secrets = String::from_utf8(private_input).unwrap();
    let auth_value: Value = serde_json::from_str(&secrets).unwrap();
    let authorization_structure: Result<Authorization<Testnet3>, Error> =
        serde_json::from_value(auth_value);

    match authorization_structure {
        Ok(auth) => {
            let is_auth_empty = auth.is_empty();
            let read_secp_private_key = fs::read("./app/secp.sec").unwrap();
            let secp_private_key = secp256k1::SecretKey::from_slice(&read_secp_private_key)
                .unwrap()
                .display_secret()
                .to_string();
            let signer_wallet = secp_private_key.parse::<LocalWallet>().unwrap();
            let ask_id = payload.ask_id;
            let value = vec![
                ethers::abi::Token::Uint(ask_id.into()),
                ethers::abi::Token::Bytes(payload.ask.prover_data.to_vec()),
            ];
            let encoded = ethers::abi::encode(&value);
            let digest = ethers::utils::keccak256(encoded);

            let signature = signer_wallet
                .sign_message(ethers::types::H256(digest))
                .await
                .unwrap();
            if is_auth_empty {
                return Ok(response(
                    "Payload is NOT valid",
                    StatusCode::OK,
                    Some(Value::String(signature.to_string())),
                ));
            } else {
                return Ok(response(
                    "Payload is valid",
                    StatusCode::OK,
                    Some(Value::String(signature.to_string())),
                ));
            }
        }
        Err(_) => {
            response(
                "The authorization input structure is invalid",
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
        .service(check_input_handler)
        .service(check_input_with_signature);
    conf.service(scope);
}
