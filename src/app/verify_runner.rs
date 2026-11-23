use crate::crypto::verifier::CertificateVerifier;
use crate::storage::key_store::KeyStore;
use crate::storage::registry_store::RegistryStore;
use std::path::Path;

pub struct VerifyRunner;

impl VerifyRunner {
    pub fn run<P: AsRef<Path>>(root: P, query: &str) -> Result<(), Box<dyn std::error::Error>> {
        let authority = KeyStore::load_or_create(&root)?;
        let registry = RegistryStore::load(&root)?;

        let mut record = registry.get(query);
        if record.is_none() {
            let q_lower = query.to_lowercase();
            for item in registry.values() {
                if item.id.to_lowercase() == q_lower
                    || item.member_no.to_string() == q_lower
                    || item.name.to_lowercase().contains(&q_lower)
                    || item.legality_id.to_lowercase().contains(&q_lower)
                {
                    record = Some(item);
                    break;
                }
            }
        }

        let Some(rec) = record else {
            eprintln!("Error: Certificate record '{}' was not found in registry.", query);
            return Err("Certificate record not found".into());
        };

        let is_valid = CertificateVerifier::verify_record(rec, &authority)?;

        println!("Certificate Verification");
        println!("Recipient: {} (Member #{})", rec.name, rec.member_no);
        println!("Credential ID: {}", rec.id);
        println!("Legality ID: {}", rec.legality_id);
        println!("Issue Date: {}", rec.date);
        println!("Algorithm: {}", rec.algorithm);
        println!("Authority Fingerprint: {}", rec.key_fingerprint);
        println!("Signature SHA3-256: {}", rec.signature_sha3_256);
        println!(
            "Verification Result: {}",
            if is_valid {
                "Valid and authentic (Verified by Rivide PQC)"
            } else {
                "Invalid signature"
            }
        );

        Ok(())
    }
}
