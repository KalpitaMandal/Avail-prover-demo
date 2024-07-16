use actix_web::{
    error,
    http::{header::ContentType, StatusCode},
    HttpResponse,
};
use bindings::shared_types::Ask;
use derive_more::{Display, Error};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct ProveAuthInputs {
    pub ask: Ask,
    pub private_input: Vec<u8>,
    pub ask_id: u64,
}

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct InputPayload {
    pub secrets: Option<String>,
}

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct EncryptedInputPayload {
    pub acl: String,
    pub encrypted_secrets: String,
    pub me_decryption_url: String,
    pub market_id: String,
}

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct AskPayload {
    pub ask: Ask,
    pub encrypted_secret: String,
    pub acl: String,
    pub ask_id: u64,
}

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct VerifyProofPayload {
    pub execution: Option<String>,
}

#[derive(Debug, Display, Error)]
pub enum InputError {
    #[display(fmt = "file not found")]
    FileNotFound,

    #[display(fmt = "execution failed")]
    ExecutionFailed,

    #[display(fmt = "invalid inputs")]
    InvalidInputs,
}

impl error::ResponseError for InputError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            InputError::FileNotFound => StatusCode::NOT_FOUND,
            InputError::ExecutionFailed => StatusCode::NOT_IMPLEMENTED,
            InputError::InvalidInputs => StatusCode::BAD_REQUEST,
        }
    }
}
