use dev_credentials::crypto::{
    AuthorityKeypair, AuthorityManager, CanonicalPayload, CertificateSigner, CertificateVerifier,
};
use dev_credentials::domain::Member;

#[test]
fn test_authority_keypair_generation_and_dto() {
    let kp = AuthorityKeypair::generate().unwrap();
    assert_eq!(kp.algorithm, "NIST FIPS 204 ML-DSA-65");
    assert!(kp.fingerprint.starts_with("PK-MLDSA65-"));

    let dto = kp.to_dto();
    let restored = AuthorityKeypair::from_dto(&dto).unwrap();
    assert_eq!(kp.fingerprint, restored.fingerprint);
    assert_eq!(kp.algorithm, restored.algorithm);
    assert_eq!(kp.public_key.as_bytes(), restored.public_key.as_bytes());
}

#[test]
fn test_canonical_payload_formatting() {
    let seed = CanonicalPayload::seed_payload("0001", "Bob Smith", "2026-11-22", "PK-TEST");
    assert_eq!(
        seed,
        "org:dev:|seq:0001|name:Bob Smith|date:2026-11-22|authority:PK-TEST"
    );

    let final_p = CanonicalPayload::final_payload(
        "RVD-DEV-0001-ABCD",
        "Bob Smith",
        "2026-11-22",
        "RVD-65:ABCD-1234-5678",
        "Founder Name",
        "Founder Role",
    );
    assert_eq!(
        final_p,
        "org:dev:|cred_id:RVD-DEV-0001-ABCD|name:Bob Smith|date:2026-11-22|legality:RVD-65:ABCD-1234-5678|signatory:Founder Name|role:Founder Role"
    );
}

#[test]
fn test_signing_and_verification() {
    let authority = AuthorityKeypair::generate().unwrap();
    let member = Member {
        id: "".to_string(),
        name: "Test Recipient".to_string(),
        date: Some("2026-11-22".to_string()),
        legality_id: None,
        member_no: None,
        algorithm: None,
        signature_sha3_256: None,
        status: None,
    };

    let record = CertificateSigner::sign_member(&member, 0, &authority).unwrap();
    assert_eq!(record.name, "Test Recipient");
    assert!(record.id.starts_with("RVD-DEV-0001-"));
    assert!(record.legality_id.starts_with("RVD-65:"));

    let is_valid = CertificateVerifier::verify_record(&record, &authority).unwrap();
    assert!(is_valid);

    let mut tampered = record.clone();
    tampered.canonical_payload = "tampered payload".to_string();
    let is_tampered_valid = CertificateVerifier::verify_record(&tampered, &authority).unwrap();
    assert!(!is_tampered_valid);
}

#[test]
fn test_authority_manager_fingerprint() {
    let kp = AuthorityKeypair::generate().unwrap();
    let fp = AuthorityManager::compute_fingerprint(kp.public_key.as_bytes());
    assert_eq!(fp, kp.fingerprint);
    assert!(AuthorityManager::matches_fingerprint(&kp, &fp));
    assert!(!AuthorityManager::matches_fingerprint(&kp, "PK-INVALID"));
}
