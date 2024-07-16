use actix_web::{get, http::StatusCode, post, web, Responder};
use aleo_rust::{Testnet3, Execution};
use ethers::{signers::{LocalWallet, Signer}, types::U256};
use serde_json::{Error, Value};
use ecies::{PublicKey, SecretKey};
use secp256k1::Secp256k1;
use snarkvm_synthesizer::Authorization;
use std::{fs, str::FromStr};

use crate::{model, prover, response::response, secret_inputs_helpers};

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
async fn check_input_handler(payload: web::Json<model::InputPayload>) -> impl Responder {
    let private_input = payload.clone().secrets.unwrap();
    let auth_value: Value = serde_json::from_str(&private_input).unwrap();
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
async fn check_input_with_signature(payload: web::Json<model::AskPayload>) -> impl Responder {
    let encrypted_input = payload.clone().encrypted_secret;
    let private_input = hex::decode(encrypted_input).unwrap();
    let acl = hex::decode(payload.clone().acl).unwrap();
    let market_id = payload.clone().ask.market_id;
    let read_secp_private_key = fs::read("./app/secp.sec").unwrap();
    let secp_private_key = secp256k1::SecretKey::from_slice(&read_secp_private_key)
        .unwrap()
        .display_secret()
        .to_string();
    let signer_wallet = secp_private_key.parse::<LocalWallet>().unwrap();
    let key = hex::decode(secp_private_key).unwrap();
    let secret = secret_inputs_helpers::decrypt_data_with_ecies_and_aes(&private_input, &acl, &key, market_id);

    match secret {
        Ok(secret_input) => {
            let decrypted_secret = String::from_utf8(secret_input).unwrap();
            let auth_value: Value = serde_json::from_str(&decrypted_secret).unwrap();
            let authorization_structure: Result<Authorization<Testnet3>, Error> =
                serde_json::from_value(auth_value);

            match authorization_structure {
                Ok(auth) => {
                    let is_auth_empty = auth.is_empty();
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
        Err(_) => {
            response(
                "The secret encyrption could not be decrypted",
                StatusCode::BAD_REQUEST,
                None,
            );
            return Err(model::InputError::InvalidInputs);
        }
    }
}

#[post("/checkEncryptedInputs")]
async fn check_encrypted_input(payload: web::Json<model::EncryptedInputPayload>) -> impl Responder {
    let encrypted_data = payload.clone();
    let encrypted_input = encrypted_data.encrypted_secrets;
    let private_input = hex::decode(encrypted_input).unwrap();
    let acl = hex::decode(encrypted_data.acl).unwrap();
    let read_market_id = encrypted_data.market_id;
    let market_id = U256::from_str(&read_market_id).unwrap();
    let read_secp_private_key = fs::read("./app/secp.sec").unwrap();
    let secp_private_key = secp256k1::SecretKey::from_slice(&read_secp_private_key)
        .unwrap()
        .display_secret()
        .to_string();
    let pub_key = hex::encode(secp_private_key.parse::<LocalWallet>().unwrap().address());
    log::info!("Public key: {:?}", pub_key.clone());
    let private_key = hex::decode(secp_private_key.clone()).unwrap();
    let private_key: &[u8; 32] = private_key.as_slice().try_into().unwrap();
    let sk = SecretKey::parse(private_key).unwrap();
    let public_key = PublicKey::from_secret_key(&sk);
    let public_key = public_key.serialize_compressed();
    let encoded_key = hex::encode(public_key);
    let formated_ecies_public_key = "0x".to_string() + &encoded_key;
    log::info!("Ecies public key: {:?}", formated_ecies_public_key);

    let key = hex::decode(secp_private_key).unwrap();
    let secret = secret_inputs_helpers::decrypt_data_with_ecies_and_aes(&private_input, &acl, &key, market_id);
    
    match secret {
        Ok(secret_input) => {
            let decrypted_secret = String::from_utf8(secret_input).unwrap();
            let auth_value: Value = serde_json::from_str(&decrypted_secret).unwrap();
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
        Err(e) => {
            log::info!("Error: {:?}", e);
            response(
                "The secret encyrption could not be decrypted",
                StatusCode::BAD_REQUEST,
                None,
            );
            return Err(model::InputError::InvalidInputs);
        }
    }
}

#[post("/verifyInputsAndProof")]
async fn verify_inputs_and_proof(payload: web::Json<model::VerifyProofPayload>) -> impl Responder {
    let private_input = payload.clone().execution.unwrap();
    let auth_value: Value = serde_json::from_str(&private_input).unwrap();
    let execution_structure: Result<Execution<Testnet3>, Error> =
        serde_json::from_value(auth_value);

    match execution_structure {
        Ok(exec) => {
            let verification_result = prover::verify_execution_proof(exec).await.unwrap();
            if verification_result {
                return Ok(response("Generated proof is valid", StatusCode::OK, None));
            } else {
                return Ok(response("Generated proof is NOT valid", StatusCode::OK, None));
            }
        }
        Err(_) => {
            response(
                "The execution input structure is invalid",
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
        .service(check_input_with_signature)
        .service(check_encrypted_input)
        .service(verify_inputs_and_proof);
    conf.service(scope);
}
