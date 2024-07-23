use actix_web::{get, http::StatusCode, post, web, HttpResponse, Responder};
use aleo_rust::{Execution, Testnet3};
use ethers::{
    core::k256::ecdsa::SigningKey,
    signers::{LocalWallet, Signer, Wallet},
};
use serde::{Deserialize, Serialize};
use serde_json::{Error, Value};
use snarkvm_synthesizer::Authorization;
use std::{fs, str::FromStr};

use crate::{
    model::{self, AskPayload},
    prover,
    response::response,
    secret_inputs_helpers,
};

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
            Ok(response(
                "Proof generated, the proof generation time returned is in milliseconds",
                StatusCode::OK,
                Some(Value::String(proving_time)),
            ))
        }
        Err(e) => {
            response(
                "There was an issue benchmarking the proof generation time.",
                StatusCode::INTERNAL_SERVER_ERROR,
                None,
            );
            Err(e)
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
                Ok(response(
                    "Proof generated",
                    StatusCode::OK,
                    Some(Value::String(encoded_bytes.to_string())),
                ))
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
        Err(e) => Err(e),
    }
}

#[post("/checkInput")]
async fn check_input_handler(payload: web::Json<model::InputPayload>) -> impl Responder {
    let private_input = payload.clone().secrets.unwrap();
    let auth_value: Value = serde_json::from_str(&private_input).unwrap();
    let authorization_structure: Result<Authorization<Testnet3>, Error> =
        serde_json::from_value(auth_value);

    check_authorization(authorization_structure, None, None).await
}

#[post("/checkInputWithSignature")]
async fn check_input_with_signature(payload: web::Json<model::AskPayload>) -> impl Responder {
    let encrypted_input = payload.clone().encrypted_secret;
    let private_input = hex::decode(encrypted_input).unwrap();
    let acl = hex::decode(payload.clone().acl).unwrap();
    let market_id = payload.clone().ask.market_id;

    let secret_input = match secret_inputs_helpers::decrypt_data_with_ecies_and_aes(
        &private_input,
        &acl,
        &get_secp_private_key(),
        market_id,
    ) {
        Ok(data) => data,
        Err(_) => {
            return response(
                "The secret encryption could not be decrypted",
                StatusCode::BAD_REQUEST,
                None,
            );
        }
    };

    let signer_wallet = get_signer();
    let decrypted_secret = String::from_utf8(secret_input).unwrap();
    let auth_value = match serde_json::from_str(&decrypted_secret) {
        Ok(data) => data,
        Err(_) => {
            return response(
                "Payload is NOT valid",
                StatusCode::OK,
                Some(Value::String(
                    generate_invalid_input_attestation(payload.0, signer_wallet).await,
                )),
            );
        }
    };

    let authorization_structure: Result<Authorization<Testnet3>, Error> =
        serde_json::from_value(auth_value);

    check_authorization(
        authorization_structure,
        Some(payload.0),
        Some(signer_wallet),
    )
    .await
}

#[post("/checkEncryptedInputs")]
async fn check_encrypted_input(payload: web::Json<model::EncryptedInputPayload>) -> impl Responder {
    #[derive(Deserialize, Serialize)]
    pub struct DecryptRequest {
        market_id: String,
        private_input: String,
        acl: String,
        signature: String,
        ivs_pubkey: String,
    }

    let payload = payload.0;
    let (signature, secp_pub_key) = {
        let message = &payload.market_id;
        let signer_wallet = get_signer();
        let digest = ethers::utils::keccak256(message);

        let read_secp_pub_key = fs::read("./app/secp.pub").unwrap();
        (
            signer_wallet
                .sign_message(ethers::types::H256(digest))
                .await
                .unwrap()
                .to_string(),
            read_secp_pub_key,
        )
    };
    let decrypt_request_payload = DecryptRequest {
        market_id: payload.market_id,
        private_input: hex::encode(payload.encrypted_secrets),
        acl: hex::encode(payload.acl),
        signature,
        ivs_pubkey: hex::encode(secp_pub_key),
    };

    let client = reqwest::Client::new();
    let api_response = client
        .post(&payload.me_decryption_url)
        .json(&decrypt_request_payload)
        .send()
        .await
        .expect("Failed to send request");

    if api_response.status().is_success() {
        #[derive(Deserialize, Debug)]
        pub struct GetRequestResponse {
            encrypted_data: String,
        }

        let response_payload: GetRequestResponse = api_response
            .json()
            .await
            .expect("Failed to deserialize response");

        let encrypted_data = hex::decode(response_payload.encrypted_data).unwrap();
        let decrypted_data =
            secret_inputs_helpers::decrypt_ecies(&get_secp_private_key(), &encrypted_data).unwrap();

        let authorization_structure: Result<Authorization<Testnet3>, Error> = {
            let decrypted_secret = String::from_utf8(decrypted_data).unwrap();
            let auth_value = match serde_json::from_str(&decrypted_secret) {
                Ok(data) => data,
                Err(_) => {
                    return response("Decrypted Data is not valid", StatusCode::OK, None);
                }
            };
            serde_json::from_value(auth_value)
        };

        check_authorization(authorization_structure, None, None).await
    } else {
        response(
            "Could not fetch info from matching engine",
            StatusCode::FAILED_DEPENDENCY,
            None,
        )
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
                Ok(response("Generated proof is valid", StatusCode::OK, None))
            } else {
                Ok(response(
                    "Generated proof is NOT valid",
                    StatusCode::OK,
                    None,
                ))
            }
        }
        Err(_) => {
            response(
                "The execution input structure is invalid",
                StatusCode::BAD_REQUEST,
                None,
            );
            Err(model::InputError::InvalidInputs)
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

async fn generate_invalid_input_attestation(
    payload: AskPayload,
    signer_wallet: Wallet<SigningKey>,
) -> String {
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

    return signature.to_string();
}

fn get_signer() -> Wallet<SigningKey> {
    let secp_private_key = secp256k1::SecretKey::from_slice(&get_secp_private_key())
        .unwrap()
        .display_secret()
        .to_string();
    secp_private_key.parse::<LocalWallet>().unwrap()
}

fn get_secp_private_key() -> Vec<u8> {
    fs::read("./app/secp.sec").unwrap()
}

async fn check_authorization(
    authorization_structure: Result<Authorization<Testnet3>, Error>,
    ask_payload: Option<AskPayload>,
    signer_wallet: Option<Wallet<SigningKey>>,
) -> HttpResponse {
    match authorization_structure {
        Ok(auth) => {
            let is_auth_empty = auth.is_empty();

            if is_auth_empty {
                if ask_payload.is_some() && signer_wallet.is_some() {
                    return response(
                        "Payload is NOT valid",
                        StatusCode::OK,
                        Some(Value::String(
                            generate_invalid_input_attestation(
                                ask_payload.unwrap(),
                                signer_wallet.unwrap(),
                            )
                            .await,
                        )),
                    );
                } else {
                    return response("Payload is NOT valid", StatusCode::OK, None);
                }
            } else {
                return response("Payload is valid", StatusCode::OK, None);
            }
        }
        Err(_) => {
            return response(
                "The authorization input structure is invalid",
                StatusCode::BAD_REQUEST,
                None,
            );
        }
    }
}
