#[derive(Clone, Default, Debug)]
pub struct RequestMetadata {
    pub request_id: String,
    pub request_chains: Vec<String>,
}

#[derive(Clone, Default, Debug)]
pub struct IncomingRequest {
    pub service: String,
    pub action: String,
    pub body: String,
}

#[derive(Clone, Default, Debug)]
pub struct Context {
    pub metadata: RequestMetadata,
    pub req: IncomingRequest,
}