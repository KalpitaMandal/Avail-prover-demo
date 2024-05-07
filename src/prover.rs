use actix_web::HttpResponse;
use aleo_rust::{
    AleoAPIClient, Encryptor, ProgramManager,
    snarkvm_types::{PrivateKey, Testnet3, Program, Process, Proof},
    ProvingKey, AleoV0, VerifyingKey, Identifier, Field
};
use serde_json::json;
use std::{fs, str::FromStr, time::Instant};
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
            let verification_result = verify_execution(
                program,
                &payload.clone().function_name, 
                execution.clone().execution_proof().unwrap()
            );

            match verification_result {
                Ok(verified) => {
                    let execution_result = json!({
                        "id": execution.clone().execution_id().unwrap().to_string(),
                        "proof": execution.clone().execution_proof().unwrap(),
                        "verify": true,
                        "total_time": total_time.as_millis().to_string() + "ms",
                        "setup_time": setup_time.as_millis().to_string() + "ms",
                        "execution_time": execution_time.as_millis().to_string() + "ms"
                    });
                    return Ok(HttpResponse::Ok().body(serde_json::to_string(&execution_result).unwrap()));
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }   
        Err(err) => {
            println!("Error: {:?}", err);
            return Err(error::InputError::ExecutionFailed);
        }       
    }
}

pub fn prove_assignment(
    function_name: &str,
    proving_key_input: ProvingKey<Testnet3>
) -> Result<HttpResponse, error::InputError> {
    let mut rng = &mut thread_rng();

    // Defining a simple hello program with only a hello function
    let test_program = format!("program {};\n\nfunction hello:\n    input r0 as u32.public;\n    input r1 as u32.private;\n    add r0 r1 into r2;\n    output r2 as u32.private;\n", "hello.aleo");
    let program = Program::from_str(&test_program).unwrap();

    // initializing a new process
    let mut process: Process<Testnet3> = Process::load().unwrap();
    process.add_program(&program).unwrap();

    // Check if program was added correctly
    let check_program = process.contains_program(program.id());
    assert!(check_program);

    let function = Identifier::<Testnet3>::try_from(function_name).unwrap();
    let proving_key = process.get_proving_key(
        program.id(), 
        function)
    .unwrap();

    // proving for the assignment
    let assign = proving_key.circuit.a.first().unwrap().first().unwrap().0;

    // let trial = proving_key_input.prove(
    //     "hello", 
    //     assign, 
    //     rng
    // );

    return Ok(HttpResponse::Ok().body("Success!"));
}

pub fn verify_execution(
    program: Program<Testnet3>,
    function_name: &str,
    proof_payload: &Proof<Testnet3>
) -> Result<HttpResponse, error::InputError> {
    // initializing a new process
    let mut process: Process<Testnet3> = Process::load().unwrap();
    process.add_program(&program).unwrap();

    // Check if program was added correctly
    let check_program = process.contains_program(program.id());
    assert!(check_program);

    // fetch the verifying key
    let verifying_key = process.get_verifying_key(
        program.id(), 
        function_name
    );

    let proving_key = process.get_proving_key(
        program.id(), 
        function_name
    ).unwrap();

    let params = proving_key.circuit.a.first().unwrap().first().unwrap().0;

    match verifying_key {
        Ok(vkey) => {
            let verify_result = vkey.verify(
                function_name, 
                &[params], 
                proof_payload
            );

            if verify_result {
                return Ok(HttpResponse::Ok().body("Verification valid"));
            } else {
                return Err(error::InputError::InvalidInputs);
            }
        }
        Err(_) => {
            return Err(error::InputError::InvalidInputs);
        }
    }
}
