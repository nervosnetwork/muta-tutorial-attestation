use serde::{Deserialize, Serialize};

use protocol::types::Hash;

type JsonString = String;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct AttestInfoPayload {
    pub info: JsonString,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct QueryAttestedInfoPayload {
    pub hash: Hash,
}

#[derive(Default, Deserialize, Serialize, Clone, Debug)]
pub struct QueryAttestedInfoResponse {
    pub attested_info: JsonString,
}
