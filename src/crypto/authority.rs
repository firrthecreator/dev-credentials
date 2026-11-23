use crate::crypto::keypair::AuthorityKeypair;
use rivide::crypto::Sha3;

pub struct AuthorityManager;

impl AuthorityManager {
    pub fn compute_fingerprint(public_key_bytes: &[u8]) -> String {
        let pk_hash = Sha3::sha3_256(public_key_bytes);
        let hex_hash = hex::encode(pk_hash).to_uppercase();
        format!(
            "PK-MLDSA65-{}-{}-{}-{}",
            &hex_hash[0..4],
            &hex_hash[4..8],
            &hex_hash[8..12],
            &hex_hash[12..16]
        )
    }

    pub fn matches_fingerprint(keypair: &AuthorityKeypair, fingerprint: &str) -> bool {
        keypair.fingerprint.eq_ignore_ascii_case(fingerprint)
    }
}
