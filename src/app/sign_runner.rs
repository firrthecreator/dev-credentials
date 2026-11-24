use crate::crypto::keypair::AuthorityKeypair;
use crate::crypto::signer::CertificateSigner;
use crate::storage::key_store::KeyStore;
use crate::storage::member_store::MemberStore;
use crate::storage::registry_store::RegistryStore;
use std::path::Path;

pub struct SignRunner;

impl SignRunner {
    pub fn run<P: AsRef<Path>>(root: P) -> Result<(), Box<dyn std::error::Error>> {
        let authority: AuthorityKeypair = KeyStore::load_or_create(&root)?;
        let mut members = MemberStore::load(&root)?;
        let mut registry = std::collections::BTreeMap::new();

        println!("Signing certificates using Rivide ML-DSA-65 authority");
        println!("Authority: {}", authority.authority);
        println!("Algorithm: {}", authority.algorithm);
        println!("Fingerprint: {}\n", authority.fingerprint);

        let total = members.len();
        for (idx, member) in members.iter_mut().enumerate() {
            let record = CertificateSigner::sign_member(member, idx, &authority)?;

            registry.insert(record.id.clone(), record.clone());

            member.id = record.id.clone();
            member.member_no = Some(record.member_no);
            member.legality_id = Some(record.legality_id.clone());
            member.algorithm = Some(record.algorithm.clone());
            member.signature_sha3_256 = Some(record.signature_sha3_256.clone());
            member.status = Some(record.status.clone());

            println!("[{}/{}] Signed: {}", idx + 1, total, member.name);
            println!("  Credential ID: {}", record.id);
            println!("  Legality ID: {}", record.legality_id);
            println!("  Status: Valid\n");
        }

        MemberStore::save(&root, &members)?;
        RegistryStore::save(&root, &registry)?;

        println!("Successfully signed {} member credentials.", total);

        Ok(())
    }
}
