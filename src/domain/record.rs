use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CertificateRecord {
    pub id: String,
    pub member_no: usize,
    pub name: String,
    pub date: String,
    pub legality_id: String,
    pub credit: String,
    pub algorithm: String,
    pub key_fingerprint: String,
    pub canonical_payload: String,
    pub signature_sha3_256: String,
    pub signature_base64: String,
    pub issued_at: String,
    pub status: String,
}
