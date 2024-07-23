mod handler;
mod model;
mod prover;
mod response;
mod secret_inputs_helpers;

use actix_web::{App, HttpServer};
use dotenv::dotenv;
use std::time::Duration;

use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| panic!("PORT must be provided in the .env file"))
        .parse::<u16>()
        .expect("PORT must be a valid number");

    let server = HttpServer::new(move || App::new().configure(handler::routes))
        .client_request_timeout(Duration::new(0, 0))
        .bind(("0.0.0.0", port))
        .unwrap_or_else(|_| panic!("Can not bind to {}", &port))
        .run();

    log::info!("avail-prover start on port {}", port);

    server.await
}

#[cfg(test)]
mod tests {
    use crate::{handler, model, secret_inputs_helpers};
    use actix_web::{test, App};
    use bindings::shared_types::Ask;
    use log::warn;
    use serde_json::{json, Value};
    use tokio::fs;

    #[actix_rt::test]
    async fn test_server() {
        let app = test::init_service(App::new().service(handler::test)).await;
        let req = test::TestRequest::get().uri("/test").to_request();
        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_success());

        let result = test::read_body(resp).await;
        let result_json: Value = serde_json::from_slice(&result).unwrap();
        let expected_json = json!({
            "message": "The Avail prover is running!!",
            "data": null
        });

        assert_eq!(result_json, expected_json);
    }

    #[actix_rt::test]
    async fn test_benchmark() {
        let app = test::init_service(App::new().service(handler::benchmark)).await;
        let req = test::TestRequest::get().uri("/benchmark").to_request();
        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_success());

        let result = test::read_body(resp).await;
        let result_json: Value = serde_json::from_slice(&result).unwrap();
        let expected_message =
            "Proof generated, the proof generation time returned is in milliseconds";

        assert_eq!(result_json["message"], expected_message);
        assert!(result_json["data"].is_string());
    }

    #[actix_rt::test]
    async fn test_generate_proof() {
        todo!("Implement proof generation testing")
    }

    #[actix_rt::test]
    async fn test_check_input() {
        let app = test::init_service(App::new().service(handler::check_input_handler)).await;

        let secrets = fs::read_to_string("./app/checkInput.txt").await.unwrap();
        let payload = model::InputPayload {
            secrets: Some(secrets),
        };

        let req = test::TestRequest::post()
            .uri("/checkInput")
            .set_json(&payload)
            .to_request();

        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_success());

        let result = test::read_body(resp).await;
        let result_json: serde_json::Value = serde_json::from_slice(&result).unwrap();
        let expected_json = json!({
            "message": "Payload is valid",
            "data": null
        });

        assert_eq!(result_json, expected_json);
    }

    #[actix_rt::test]
    async fn test_check_wrong_input() {
        todo!("Add cases where invalid input is provided, wrong signature is provided")
    }

    #[actix_rt::test]
    async fn test_check_input_with_signature() {
        let app = test::init_service(App::new().service(handler::check_input_with_signature)).await;
        let data_to_encrypt = fs::read("./app/checkInput.txt").await.unwrap();
        // bit un-intutive, but rn this seems only way to test
        let receiver_pub_key = fs::read("./app/secp.pub").await.unwrap();
        let encrypted_data = secret_inputs_helpers::encrypt_data_with_ecies_and_aes(
            &receiver_pub_key,
            &data_to_encrypt,
        )
        .unwrap();

        let ask: Ask = Ask {
            market_id: 1.into(),
            reward: 1.into(),
            expiry: 1.into(),
            time_taken_for_proof_generation: 1.into(),
            deadline: 1.into(),
            refund_address: "0000dead0000dead0000dead0000dead0000dead".parse().unwrap(),
            prover_data: [1, 2, 3, 4].into(),
        };
        let ask_payload = model::AskPayload {
            ask_id: 1,
            ask,
            encrypted_secret: hex::encode(encrypted_data.encrypted_data),
            acl: hex::encode(encrypted_data.acl_data),
        };

        let req = test::TestRequest::post()
            .uri("/checkInputWithSignature")
            .set_json(&ask_payload)
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        let result = test::read_body(resp).await;
        let result_json: serde_json::Value = serde_json::from_slice(&result).unwrap();
        // when payload is valid, signature is not required to be sent
        let expected_json = json!({
            "message": "Payload is valid",
            "data": null
        });
        assert_eq!(result_json, expected_json);
    }

    #[actix_rt::test]
    async fn test_check_wrong_input_with_signature() {
        let app = test::init_service(App::new().service(handler::check_input_with_signature)).await;
        let data_to_encrypt = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 2, 3, 4, 5]; // these are invalid inputs
                                                                              // bit un-intutive, but rn this seems only way to test
        let receiver_pub_key = fs::read("./app/secp.pub").await.unwrap();
        let encrypted_data = secret_inputs_helpers::encrypt_data_with_ecies_and_aes(
            &receiver_pub_key,
            &data_to_encrypt,
        )
        .unwrap();

        let ask: Ask = Ask {
            market_id: 1.into(),
            reward: 1.into(),
            expiry: 1.into(),
            time_taken_for_proof_generation: 1.into(),
            deadline: 1.into(),
            refund_address: "0000dead0000dead0000dead0000dead0000dead".parse().unwrap(),
            prover_data: [1, 2, 3, 4].into(),
        };
        let ask_payload = model::AskPayload {
            ask_id: 1,
            ask,
            encrypted_secret: hex::encode(encrypted_data.encrypted_data),
            acl: hex::encode(encrypted_data.acl_data),
        };

        let req = test::TestRequest::post()
            .uri("/checkInputWithSignature")
            .set_json(&ask_payload)
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        let result = test::read_body(resp).await;
        let result_json: serde_json::Value = serde_json::from_slice(&result).unwrap();
        // when payload is valid, signature is not required to be sent
        // below info is computed for above ask
        let expected_json = json!({
            "message": "Payload is NOT valid",
            "data": "e8ef983340f3f23cc31c1fc8daed52b1d3a2d3b06369ec29b8a549ecab17383402575c86525a07acf237cc06c30a40158672cdb30c550f32f7263f34a5d46cf11b"
        });
        assert_eq!(result_json, expected_json);
    }

    #[actix_rt::test]
    async fn test_check_encrypted_input() {
        //tough one.
        let app = test::init_service(App::new().service(handler::check_input_with_signature)).await;
        let data_to_encrypt = fs::read("./app/checkInput.txt").await.unwrap();

        warn!("Matching Engine IP hardcoded, it should be fetched from somewhere else");

        let ivs_pubkey = fs::read("./app/secp.pub").await.unwrap();
        let encrypted_data =
            secret_inputs_helpers::encrypt_data_with_ecies_and_aes(&ivs_pubkey, &data_to_encrypt)
                .unwrap();

        let payload: model::EncryptedInputPayload = model::EncryptedInputPayload {
            acl: hex::encode(encrypted_data.acl_data),
            encrypted_secrets: hex::encode(encrypted_data.encrypted_data),
            me_decryption_url: "http://13.201.131.193:3000/decryptRequest".into(),
            market_id: "19".into(),
        };

        let req = test::TestRequest::post()
            .uri("/checkEncryptedInputs")
            .set_json(&payload)
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        let result = test::read_body(resp).await;
        let result_json: serde_json::Value = serde_json::from_slice(&result).unwrap();
        // when payload is valid, signature is not required to be sent
        let expected_json = json!({
            "message": "Payload is valid",
            "data": null
        });
        assert_eq!(result_json, expected_json);
    }

    #[actix_rt::test]
    async fn test_check_encrypted_invalid_input() {
        //tough one.
        let app = test::init_service(App::new().service(handler::check_input_with_signature)).await;
        let data_to_encrypt = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0];

        warn!("Matching Engine IP hardcoded, it should be fetched from somewhere else");

        let ivs_pubkey = fs::read("./app/secp.pub").await.unwrap();
        let encrypted_data =
            secret_inputs_helpers::encrypt_data_with_ecies_and_aes(&ivs_pubkey, &data_to_encrypt)
                .unwrap();

        let payload: model::EncryptedInputPayload = model::EncryptedInputPayload {
            acl: hex::encode(encrypted_data.acl_data),
            encrypted_secrets: hex::encode(encrypted_data.encrypted_data),
            me_decryption_url: "http://13.201.131.193:3000/decryptRequest".into(),
            market_id: "19".into(),
        };

        let req = test::TestRequest::post()
            .uri("/checkEncryptedInputs")
            .set_json(&payload)
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        let result = test::read_body(resp).await;
        let result_json: serde_json::Value = serde_json::from_slice(&result).unwrap();
        // when payload is valid, signature is not required to be sent
        let expected_json = json!({
            "message": "Payload is NOT valid",
            "data": null
        });
        assert_eq!(result_json, expected_json);
    }
}
