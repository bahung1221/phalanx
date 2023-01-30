use serde::{Serialize, Deserialize};

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct RequestMetadata {
    pub request_id: String,
    pub request_chains: Vec<String>,
}

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct IncomingRequest {
    pub service: String,
    pub action: String,
    pub body: String,
}

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct Context {
    pub metadata: RequestMetadata,
    pub req: IncomingRequest,
}

impl Context {
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}