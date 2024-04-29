use actix_web::{get, App, HttpResponse, HttpServer};
use std::{fs, str::FromStr};
use aleo_rust::{
    AleoAPIClient, Encryptor, ProgramManager,
    snarkvm_types::{PrivateKey, Testnet3, Program},
    ProvingKey, AleoV0
  };
use serde::{Deserialize, Serialize};
use serde_json::json;
use rand::thread_rng;

mod helpers;
mod prover;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct ProverConfig {
    private_key: String,
    query_url: String
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    println!("Starting Aleo prover...");
    HttpServer::new(|| {
        App::new()
            .service(prove_transition)
            .service(execute)
            .service(execute_offline)
            .service(welcome)
    })
    .bind(("0.0.0.0", 3030))?
    .run()
    .await
}

#[get("/welcome")]
async fn welcome() -> Result<HttpResponse, helpers::error::InputError> {
    // Create a client that interacts with the testnet3 program
    let api_client = AleoAPIClient::<Testnet3>::testnet3();

    // Check if correct program is getting fetched 
    let credits = api_client.get_program("credits.aleo").unwrap();
    println!("Credits program: {credits:?}");

    return Ok(HttpResponse::Ok().body("Welcome!..."));
}

// To execute deployed programs
#[get("/execute")]
async fn execute() -> Result<HttpResponse, helpers::error::InputError> {
    let config_path = "./config.json".to_string();
    let content = fs::read_to_string(config_path).unwrap();
    let config: ProverConfig = serde_json::from_str(&content).unwrap();
    let api_client = AleoAPIClient::<Testnet3>::testnet3();
    let private_key = PrivateKey::<Testnet3>::from_str(&config.private_key).unwrap();
    let private_key_ciphertext = Encryptor::<Testnet3>::encrypt_private_key_with_secret(&private_key, "password").unwrap();

    let mut program_manager = ProgramManager::<Testnet3>::new(
        None, 
        Some(private_key_ciphertext), 
        Some(api_client), 
        None, 
        false).unwrap();

    // execute program onchain
    let results = program_manager.execute_program(
        "credits.aleo", 
        "transfer_public", 
        ["aleo1va0hzrcsj569gz0gd0mvue7xk54vku626nsmvl86s7j490x7yupq89l82z", "3u64"].into_iter(), 
        0, 
        None, 
        Some("password"), 
        None
    );

    match results {
        Ok(txhash) => {
            return Ok(HttpResponse::Ok().body(txhash));
        }   
        Err(_) => {
            return Err(helpers::error::InputError::PayloadNotValid);
        }       
    }
}

// To execute local programs 
#[get("/executeOffline")]
async fn execute_offline() -> Result<HttpResponse, helpers::error::InputError> {
    let config_path = "./config.json".to_string();
    let content = fs::read_to_string(config_path).unwrap();
    let config: ProverConfig = serde_json::from_str(&content).unwrap();
    let api_client = AleoAPIClient::<Testnet3>::testnet3();
    let private_key = PrivateKey::<Testnet3>::from_str(&config.private_key).unwrap();
    let private_key_ciphertext = Encryptor::<Testnet3>::encrypt_private_key_with_secret(&private_key, "password").unwrap();

    let program_manager = ProgramManager::<Testnet3>::new(
        None, 
        Some(private_key_ciphertext), 
        Some(api_client), 
        None, 
        false).unwrap();

    let test_program = format!("program {};\n\nfunction hello:\n    input r0 as u32.public;\n    input r1 as u32.private;\n    add r0 r1 into r2;\n    output r2 as u32.private;\n", "hello.aleo");
    let program = Program::from_str(&test_program).unwrap();

    // execute program offline
    let results = program_manager.execute_program_offline::<AleoV0>(
        &private_key,
        &program, 
        "hello", 
        &[], 
        ["5u32", "3u32"].into_iter(), 
        true, 
        &config.query_url
    );

    match results {
        Ok(execution) => {
            let execution_proof = execution.clone().execution_proof().unwrap().to_string();
            let execution_id = execution.clone().execution_id().unwrap().to_string();
            let execution_result = json!({
                "id": execution_id,
                "proof": execution_proof
            });
            println!("Execution result: {:?}", execution.clone().execution_proof());
            return Ok(HttpResponse::Ok().body(serde_json::to_string(&execution_result).unwrap()));
        }   
        Err(err) => {
            println!("Error: {:?}", err);
            return Err(helpers::error::InputError::PayloadNotValid);
        }       
    }
}

#[get("/prove")]
async fn prove_transition() -> Result<HttpResponse, helpers::error::InputError> {
    let proving_key_path = "../keys/transfer_public.prover".to_string();
    let alt_proving_key_path = "./keys/transfer_public.prover".to_string();
    let file_content =
        fs::read(&proving_key_path).or_else(|_| fs::read(&alt_proving_key_path));
    match file_content {
        Ok(key) => {
            let mut _rng = thread_rng();
            let key_string = String::from_utf8(key).unwrap();
            let _proving_key: ProvingKey<Testnet3> = ProvingKey::from_str(&key_string).unwrap();
            // let prove = _proving_key.prove(
            //     "transfer_public", 
            //     assignment, 
            //     &mut rng
            // );
            println!("Proving key: success");
        }
        Err(_) => {
            return Err(helpers::error::InputError::FileNotFound);
        }
    }
    return Ok(HttpResponse::Ok().body("Proving your transaction, please wait!"));
}
