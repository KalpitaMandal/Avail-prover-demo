use actix_web::HttpResponse;
use aleo_rust::{
    AleoAPIClient, Encryptor, ProgramManager,
    snarkvm_types::{PrivateKey, Testnet3, Program},
    ProvingKey, AleoV0
};
use serde_json::json;
use std::{str::FromStr, time::Instant};
use crate::helpers::{error, input};

// TODO: Add verification logic & use proving key
pub fn execute_credits_transfer_public(
    config: input::ProverConfig,
    payload: input::ExecutePayload
) -> Result<HttpResponse, error::InputError> {
    let total_now = Instant::now();
    let setup_now = Instant::now();
    let api_client = AleoAPIClient::<Testnet3>::testnet3();
    let private_key = PrivateKey::<Testnet3>::from_str(&config.private_key).unwrap();
    let private_key_ciphertext = Encryptor::<Testnet3>::encrypt_private_key_with_secret(
        &private_key, 
        &config.secret
    ).unwrap();

    let mut program_manager = ProgramManager::<Testnet3>::new(
        None, 
        Some(private_key_ciphertext), 
        Some(api_client), 
        None, 
        false).unwrap();

    // Calculating the time taken for the setup
    let setup_time = setup_now.elapsed();
    let execution_now = Instant::now();

    // execute program onchain
    let results = program_manager.execute_program(
        payload.program_id, 
        payload.function, 
        [payload.input_add, payload.input_amt].into_iter(), 
        0, 
        None, 
        Some(&config.secret), 
        None
    );

    match results {
        Ok(txhash) => {
            let execution_time = execution_now.elapsed();
            let total_time = total_now.elapsed();
            let response_body = json!({
                "txnHash": txhash,
                "total_time": total_time.as_millis().to_string() + "ms",
                "setup_time": setup_time.as_millis().to_string() + "ms",
                "execution_time": execution_time.as_millis().to_string() + "ms"
            });
            return Ok(HttpResponse::Ok().body(serde_json::to_string(&response_body).unwrap()));
        }
        Err(_) => {
            return Err(error::InputError::ExecutionFailed);
        }
    }
}

pub fn execute_offline_hello(
    config: input::ProverConfig,
    payload: input::ExecuteOfflinePayload
) -> Result<HttpResponse, error::InputError> {
    let total_now = Instant::now();
    let setup_now = Instant::now();
    let api_client = AleoAPIClient::<Testnet3>::testnet3();
    let private_key = PrivateKey::<Testnet3>::from_str(&config.private_key).unwrap();
    let private_key_ciphertext = Encryptor::<Testnet3>::encrypt_private_key_with_secret(
        &private_key, 
        &config.secret
    ).unwrap();

    let program_manager = ProgramManager::<Testnet3>::new(
        None, 
        Some(private_key_ciphertext), 
        Some(api_client), 
        None, 
        false).unwrap();

    // Defining a simple hello program with only a hello function
    let test_program = format!("program {};\n\nfunction hello:\n    input r0 as u32.public;\n    input r1 as u32.private;\n    add r0 r1 into r2;\n    output r2 as u32.private;\n", "hello.aleo");
    let program = Program::from_str(&test_program).unwrap();

    // Calculating the time taken for the setup
    let setup_time = setup_now.elapsed();
    let execution_now = Instant::now();

    // execute program offline
    let results = program_manager.execute_program_offline::<AleoV0>(
        &private_key,
        &program, 
        payload.function_name, 
        &[], 
        [payload.input_r0, payload.input_r1].into_iter(), 
        true, 
        &config.query_url
    );

    match results {
        Ok(execution) => {
            let execution_time = execution_now.elapsed();
            let total_time = total_now.elapsed();
            let execution_proof = execution.clone().execution_proof().unwrap().to_string();
            let execution_id = execution.clone().execution_id().unwrap().to_string();
            let execution_result = json!({
                "id": execution_id,
                "proof": execution_proof,
                "total_time": total_time.as_millis().to_string() + "ms",
                "setup_time": setup_time.as_millis().to_string() + "ms",
                "execution_time": execution_time.as_millis().to_string() + "ms"
            });
            return Ok(HttpResponse::Ok().body(serde_json::to_string(&execution_result).unwrap()));
        }   
        Err(err) => {
            println!("Error: {:?}", err);
            return Err(error::InputError::ExecutionFailed);
        }       
    }
}