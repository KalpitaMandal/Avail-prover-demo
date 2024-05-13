use actix_web::{get, post, web, App, HttpResponse, HttpServer};
use std::{fs, time::Instant};
use aleo_rust::{
    AleoAPIClient,
    snarkvm_types::Testnet3
  };
use serde_json::{Error, json};

mod helpers;
mod prover;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    println!("Starting Aleo prover...");
    HttpServer::new(|| {
        App::new()
            .service(execute_multi)
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
    let total_now = Instant::now();
    let setup_now = Instant::now();
    // Create a client that interacts with the testnet3 program
    let api_client = AleoAPIClient::<Testnet3>::testnet3();

    // Calculating the time taken for the setup
    let setup_time = setup_now.elapsed();
    let execution_now = Instant::now();

    // Check if correct program is getting fetched 
    let credits = api_client.get_program("credits.aleo").unwrap();
    let execution_time = execution_now.elapsed();
    let total_time = total_now.elapsed();
    println!("Credits program: {credits:?}");


    let response_body = json!({
        "total_time": total_time.as_millis().to_string() + "ms",
        "setup_time": setup_time.as_millis().to_string() + "ms",
        "execution_time": execution_time.as_millis().to_string() + "ms"
    });
    return Ok(HttpResponse::Ok().body(serde_json::to_string(&response_body).unwrap()));
}

// To execute deployed programs
#[post("/execute")]
async fn execute(
    payload: web::Json<helpers::input::ExecutePayload>,
) -> Result<HttpResponse, helpers::error::InputError> {
    let config_path = "./config.json".to_string();
    let config_content = fs::read_to_string(config_path);

    match config_content {
        Ok(content) => {
            let config_json: Result<helpers::input::ProverConfig, Error> = serde_json::from_str(&content);
            match config_json {
                Ok(config) => {
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
                Err(_) => {
                    return Err(helpers::error::InputError::BadConfigData);
                }
            }
        }
        Err(_) => {
            return Err(helpers::error::InputError::FileNotFound);
        }
    }
}

// To execute local programs 
#[post("/executeOffline")]
async fn execute_offline(
    payload: web::Json<helpers::input::ExecuteOfflinePayload>,
) -> Result<HttpResponse, helpers::error::InputError> {
    let config_path = "./config.json".to_string();
    let config_content = fs::read_to_string(config_path);

    match config_content {
        Ok(content) => {
            let config_json: Result<helpers::input::ProverConfig, Error> = serde_json::from_str(&content);
            match config_json {
                Ok(config) => {
                    let execute_result = prover::execute_offline_hello(
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
                Err(_) => {
                    return Err(helpers::error::InputError::BadConfigData);
                }
            }
        }
        Err(_) => {
            return Err(helpers::error::InputError::FileNotFound);
        }
    }
}

#[get("/prove")]
async fn prove_transition() -> Result<HttpResponse, helpers::error::InputError> {
    let config_path = "./config.json".to_string();
    let config_content = fs::read_to_string(config_path);

    match config_content {
        Ok(content) => {
            let config_json: Result<helpers::input::ProverConfig, Error> = serde_json::from_str(&content);
            match config_json {
                Ok(config) => {
                    let prove_result = prover::prove_authorization(config.private_key);

                    match prove_result {
                        Ok(result) => {
                            return Ok(result);
                        }
                        Err(e) => {
                            return Err(e);
                        }
                    }
                }
                Err(_) => {
                    return Err(helpers::error::InputError::BadConfigData);
                }
            }
        }
        Err(_) => {
            return Err(helpers::error::InputError::FileNotFound);
        }
    }
}


#[get("/executeMulti")]
async fn execute_multi() -> Result<HttpResponse, helpers::error::InputError> {
    let config_path = "./config.json".to_string();
    let config_content = fs::read_to_string(config_path);

    match config_content {
        Ok(content) => {
            let config_json: Result<helpers::input::ProverConfig, Error> = serde_json::from_str(&content);
            match config_json {
                Ok(config) => {
                    let prove_result = prover::execute_offline_multi(config);

                    match prove_result {
                        Ok(result) => {
                            return Ok(result);
                        }
                        Err(e) => {
                            return Err(e);
                        }
                    }
                }
                Err(_) => {
                    return Err(helpers::error::InputError::BadConfigData);
                }
            }
        }
        Err(_) => {
            return Err(helpers::error::InputError::FileNotFound);
        }
    }
}
