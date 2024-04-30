use actix_web::{body::BoxBody, http::header::ContentType, HttpRequest, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProverConfig {
    pub private_key: String,
    pub query_url: String,
    pub secret: String,
}

// The payload for the POST request
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ExecutePayload {
    pub program_id: String,
    pub function: String,
    pub input_add: String,
    pub input_amt: String,
}

// Responder
impl Responder for ExecutePayload {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();

        // Create response and set content type
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ExecuteOfflinePayload {
    pub function_name: String,
    pub input_r0: String,
    pub input_r1: String
}
