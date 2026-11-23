use dev_credentials::storage::{
    ConfigStore, KeyStore, MemberStore, PathResolver, RegistryStore,
};
use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;

#[test]
fn test_path_resolver() {
    let root = PathBuf::from("/test/root");
    assert_eq!(
        PathResolver::data_dir(&root),
        PathBuf::from("/test/root/data")
    );
    assert_eq!(
        PathResolver::config_file(&root),
        PathBuf::from("/test/root/data/config.json")
    );
    assert_eq!(
        PathResolver::members_file(&root),
        PathBuf::from("/test/root/data/members.json")
    );
    assert_eq!(
        PathResolver::registry_file(&root),
        PathBuf::from("/test/root/data/db_registry.json")
    );
    assert_eq!(
        PathResolver::keys_dir(&root),
        PathBuf::from("/test/root/keys/authority")
    );
    assert_eq!(
        PathResolver::logos_dir(&root),
        PathBuf::from("/test/root/assets/logos")
    );
    assert_eq!(
        PathResolver::assets_dir(&root),
        PathBuf::from("/test/root/assets")
    );
}

#[test]
fn test_store_roundtrips() {
    let temp_dir = std::env::temp_dir().join(format!("dev_cred_test_{}", std::process::id()));
    fs::create_dir_all(&temp_dir).unwrap();

    let config = ConfigStore::load(&temp_dir).unwrap();
    ConfigStore::save(&temp_dir, &config).unwrap();
    let reloaded_config = ConfigStore::load(&temp_dir).unwrap();
    assert_eq!(config.circle_name, reloaded_config.circle_name);

    let keypair = KeyStore::load_or_create(&temp_dir).unwrap();
    let reloaded_kp = KeyStore::load_or_create(&temp_dir).unwrap();
    assert_eq!(keypair.fingerprint, reloaded_kp.fingerprint);

    let members = vec![dev_credentials::domain::Member {
        id: "RVD-DEV-0001-A1B2".to_string(),
        name: "Test User".to_string(),
        date: Some("2026-11-22".to_string()),
        legality_id: None,
        member_no: Some(1),
        algorithm: None,
        signature_sha3_256: None,
        status: None,
    }];
    MemberStore::save(&temp_dir, &members).unwrap();
    let reloaded_members = MemberStore::load(&temp_dir).unwrap();
    assert_eq!(members, reloaded_members);

    let mut registry = BTreeMap::new();
    registry.insert(
        "KEY1".to_string(),
        dev_credentials::domain::CertificateRecord {
            id: "ID1".to_string(),
            member_no: 1,
            name: "Test".to_string(),
            date: "2026".to_string(),
            legality_id: "L1".to_string(),
            credit: "Cred".to_string(),
            algorithm: "Algo".to_string(),
            key_fingerprint: "FP".to_string(),
            canonical_payload: "Pay".to_string(),
            signature_sha3_256: "Hash".to_string(),
            signature_base64: "Sig".to_string(),
            issued_at: "Now".to_string(),
            status: "OK".to_string(),
        },
    );
    RegistryStore::save(&temp_dir, &registry).unwrap();
    let reloaded_registry = RegistryStore::load(&temp_dir).unwrap();
    assert_eq!(registry.len(), reloaded_registry.len());

    let _ = fs::remove_dir_all(&temp_dir);
}
