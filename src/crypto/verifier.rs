use crate::crypto::keypair::AuthorityKeypair;
use crate::domain::record::CertificateRecord;
use base64::engine::general_purpose::STANDARD as BASE64;
use base64::Engine;
use rivide::dsa::{MlDsa65, MlDsa65Signature};

pub struct CertificateVerifier;

impl CertificateVerifier {
    pub fn verify_record(
        record: &CertificateRecord,
        authority: &AuthorityKeypair,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig_bytes = BASE64.decode(&record.signature_base64)?;
        if sig_bytes.len() != 3309 {
            return Ok(false);
        }

        let mut sig_arr = [0u8; 3309];
        sig_arr.copy_from_slice(&sig_bytes);
        let sig = MlDsa65Signature::from_bytes(sig_arr);

        let ok = MlDsa65::verify(
            &sig,
            record.canonical_payload.as_bytes(),
            &authority.public_key,
        );

        Ok(ok)
    }
}
