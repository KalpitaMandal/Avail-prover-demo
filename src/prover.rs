use actix_web::HttpResponse;
use aleo_rust::{
    AleoAPIClient, Encryptor, ProgramManager,
    snarkvm_types::{PrivateKey, Testnet3, Program, Process},
    AleoV0, Identifier, Locator, Query, BlockStore, BlockMemory
};
use ethers::signers::{LocalWallet, Signer};
use secp256k1;
use serde_json::json;
use std::{fs, str::FromStr, time::Instant};
use crate::model;
use rand::thread_rng;

pub fn execute_credits_transfer_public(
    config: model::ProverConfig,
    payload: model::ProverInputs
) -> Result<HttpResponse, model::InputError> {
    let total_now = Instant::now();
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

    let execution_now = Instant::now();

    // execute program onchain
    let results = program_manager.execute_program(
        "credits.aleo", 
        "transfer_public", 
        payload.public_inputs.into_iter(), 
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
                "execution_time": execution_time.as_millis().to_string() + "ms"
            });
            return Ok(HttpResponse::Ok().body(serde_json::to_string(&response_body).unwrap()));
        }
        Err(_) => {
            return Err(model::InputError::ExecutionFailed);
        }
    }
}

pub fn prove_authorization(
    private_key: String,
    payload: model::ProverInputs
) -> Result<HttpResponse, model::InputError> {
    let total_now = Instant::now();
    let rng = &mut thread_rng();
    let pkey = PrivateKey::<Testnet3>::from_str(&private_key).unwrap();

    // Defining a simple hello program with only a hello function
    let program_path = "./test_hello.txt".to_string();
    let alt_program_path = "../test_hello.txt".to_string();
    let file_content =
        fs::read_to_string(program_path).or_else(|_| fs::read_to_string(alt_program_path));
    if file_content.is_err() {
        log::error!("{:#?}", file_content.err());
        return Err(model::InputError::FileNotFound);
    }
    let test_program = file_content.unwrap();
    let program = Program::from_str(&test_program).unwrap();

    // initializing a new process
    let mut process: Process<Testnet3> = Process::load().unwrap();
    process.add_program(&program).unwrap();

    // Check if program was added correctly
    let check_program = process.contains_program(program.id());
    assert!(check_program);

    let function = Identifier::<Testnet3>::try_from("hello").unwrap();
    let auth = process.authorize::<AleoV0,_>(
        &pkey, 
        program.id(), 
        function, 
        payload.public_inputs.into_iter(),
        // ["3u32", "5u32"].into_iter(), 
        rng
    ).unwrap();

    let execute_now = Instant::now();

    let (_result, mut trace) = process.execute::<AleoV0,_>(auth, rng).unwrap();

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
                // "result": serde_json::to_string(&result.outputs().to_vec()).unwrap(),
                "proof": prove.proof().unwrap(),
                "verified": true,
                "proof_time": prove_time.as_millis().to_string() + "ms",
                "total_time": total_time.as_millis().to_string() + "ms",
                "execution_time": execute_time.as_millis().to_string() + "ms"
            });
            return Ok(HttpResponse::Ok().body(serde_json::to_string(&execution_result).unwrap()));
        }
        Err(e) => {
            println!("Error: {:?}", e);
            return Err(model::InputError::InvalidInputs);
        }
    }
}

pub async fn prove_multi(
    private_key: String,
    payload: model::ProverInputs
) -> Result<HttpResponse, model::InputError> {
    let total_now = Instant::now();
    let rng = &mut thread_rng();
    let pkey = PrivateKey::<Testnet3>::from_str(&private_key).unwrap();
    let read_secp_private_key = fs::read("/app/secp.sec").unwrap();
    let secp_private_key = secp256k1::SecretKey::from_slice(&read_secp_private_key)
        .unwrap()
        .display_secret()
        .to_string();
    let signer_wallet = secp_private_key.parse::<LocalWallet>().unwrap();

    // Defining a complex program with 4 transitions
    let multi_program_path = "./multi_txn_t1.txt".to_string();
    let alt_multi_program_path = "../multi_txn_t1.txt".to_string();
    let file_content =
        fs::read_to_string(multi_program_path).or_else(|_| fs::read_to_string(alt_multi_program_path));
    if file_content.is_err() {
        log::error!("{:#?}", file_content.err());
        return Err(model::InputError::FileNotFound);
    }
    let test_program = file_content.unwrap();
    let program = Program::from_str(&test_program).unwrap();

    let helper_program_path = "./helper.txt".to_string();
    let alt_helper_program_path = "../helper.txt".to_string();
    let file_content =
        fs::read_to_string(helper_program_path).or_else(|_| fs::read_to_string(alt_helper_program_path));
    if file_content.is_err() {
        log::error!("{:#?}", file_content.err());
        return Err(model::InputError::FileNotFound);
    }
    let im_1 = file_content.unwrap();
    let im_program_1 = Program::from_str(&im_1).unwrap();

    let fees_program_path = "./fees.txt".to_string();
    let alt_fees_program_path = "../fees.txt".to_string();
    let file_content =
        fs::read_to_string(fees_program_path).or_else(|_| fs::read_to_string(alt_fees_program_path));
    if file_content.is_err() {
        log::error!("{:#?}", file_content.err());
        return Err(model::InputError::FileNotFound);
    }
    let im_2 = file_content.unwrap();
    let im_program_2 = Program::from_str(&im_2).unwrap();

    // initializing a new process
    let mut process: Process<Testnet3> = Process::load().unwrap();
    process.add_program(&im_program_1).unwrap();
    process.add_program(&im_program_2).unwrap();
    process.add_program(&program).unwrap();

    // Check if program was added correctly
    let check_program = process.contains_program(program.id());
    assert!(check_program);

    let function = Identifier::<Testnet3>::try_from("transfer_public").unwrap();
    let auth = process.authorize::<AleoV0,_>(
        &pkey, 
        program.id(), 
        function, 
        payload.public_inputs.into_iter(),
        // ["aleo1rn636g94mx3qqhf7m79nsne3llv4dqs25707yhwcrk92p0kwrc9qe392wg", "3u64", "1u64"].into_iter(), 
        rng
    ).unwrap();

    let execute_now = Instant::now();

    let (_result, mut trace) = process.execute::<AleoV0,_>(auth, rng).unwrap();

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

            let ask_id = payload.ask_id;
            let public_inputs = payload.ask.prover_data;
            let value = vec![
                ethers::abi::Token::Uint(ask_id.into()),
                ethers::abi::Token::Bytes(public_inputs.to_vec()),
            ];
            let encoded = ethers::abi::encode(&value);
            let digest = ethers::utils::keccak256(encoded);

            let signature = signer_wallet
                .sign_message(ethers::types::H256(digest))
                .await
                .unwrap();
            let execution_result = json!({
                "id": ask_id,
                // "result": serde_json::to_string(&result.outputs().to_vec()).unwrap(),
                "proof": prove.proof().unwrap(),
                "verified": true,
                "signature": Some("0x".to_owned() + &signature.to_string()),
                "proof_time": prove_time.as_millis().to_string() + "ms",
                "total_time": total_time.as_millis().to_string() + "ms",
                "execution_time": execute_time.as_millis().to_string() + "ms"
            });
            return Ok(HttpResponse::Ok().body(serde_json::to_string(&execution_result).unwrap()));
        }
        Err(e) => {
            println!("Error: {:?}", e);
            return Err(model::InputError::InvalidInputs);
        }
    }
}