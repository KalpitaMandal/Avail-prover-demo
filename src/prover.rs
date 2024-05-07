use actix_web::HttpResponse;
use aleo_rust::{
    AleoAPIClient, Encryptor, ProgramManager,
    snarkvm_types::{PrivateKey, Testnet3, Program, Process},
    AleoV0, Identifier, Locator, Query, BlockStore, BlockMemory
};
use serde_json::json;
use std::{str::FromStr, time::Instant};
use crate::helpers::{error, input};
use rand::thread_rng;

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
        payload.clone().function_name, 
        &[], 
        [payload.clone().input_r0, payload.clone().input_r1].into_iter(), 
        true, 
        &config.query_url
    );

    match results {
        Ok(execution) => {
            let execution_time = execution_now.elapsed();
            let total_time = total_now.elapsed();
            let execution_result = json!({
                "id": execution.clone().execution_id().unwrap().to_string(),
                "proof": execution.clone().execution_proof().unwrap(),
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

pub fn prove_authorization(
    private_key: String
) -> Result<HttpResponse, error::InputError> {
    let total_now = Instant::now();
    let setup_now = Instant::now();
    let rng = &mut thread_rng();
    let pkey = PrivateKey::<Testnet3>::from_str(&private_key).unwrap();

    // Defining a simple hello program with only a hello function
    let test_program = format!("program {};\n\nfunction hello:\n    input r0 as u32.public;\n    input r1 as u32.private;\n    add r0 r1 into r2;\n    output r2 as u32.private;\n", "hello.aleo");
    let program = Program::from_str(&test_program).unwrap();

    // initializing a new process
    let mut process: Process<Testnet3> = Process::load().unwrap();
    process.add_program(&program).unwrap();

    // Check if program was added correctly
    let check_program = process.contains_program(program.id());
    assert!(check_program);

    let setup_time = setup_now.elapsed();
    let auth_now = Instant::now();
    let function = Identifier::<Testnet3>::try_from("hello").unwrap();
    let auth = process.authorize::<AleoV0,_>(
        &pkey, 
        program.id(), 
        function, 
        ["3u32", "5u32"].into_iter(), 
        rng
    ).unwrap();

    let auth_time = auth_now.elapsed();
    let execute_now = Instant::now();

    let (result, mut trace) = process.execute::<AleoV0,_>(auth, rng).unwrap();

    let execute_time = execute_now.elapsed();
    let prove_now = Instant::now();

    let locator = Locator::new(*program.id(), function);
    let block_store = BlockStore::<Testnet3, BlockMemory<_>>::open(None).unwrap();
    trace.prepare(Query::from(block_store)).unwrap();
    let prove_result = trace.prove_execution::<AleoV0,_>(&locator.to_string(), rng);

    match prove_result {
        Ok(prove) => {
            let prove_time = prove_now.elapsed();
            let total_time = total_now.elapsed();
            process.verify_execution(&prove).unwrap();
            let execution_result = json!({
                "id": prove.to_execution_id().unwrap(),
                "result": serde_json::to_string(&result.outputs().to_vec()).unwrap(),
                "proof": prove.proof().unwrap(),
                "verified": true,
                "auth_time": auth_time.as_millis().to_string() + "ms",
                "proof_time": prove_time.as_millis().to_string() + "ms",
                "total_time": total_time.as_millis().to_string() + "ms",
                "setup_time": setup_time.as_millis().to_string() + "ms",
                "execution_time": execute_time.as_millis().to_string() + "ms"
            });
            return Ok(HttpResponse::Ok().body(serde_json::to_string(&execution_result).unwrap()));
        }
        Err(e) => {
            println!("Error: {:?}", e);
            return Err(error::InputError::InvalidInputs);
        }
    }
}
