use dev_credentials::cli::{OutputFormat, ThemeMode};
use dev_credentials::domain::{CertificateConfig, Member};
use dev_credentials::renderer::render_certificate;
use std::fs;

#[test]
fn test_render_certificate_all_formats() {
    let temp_out = std::env::temp_dir().join(format!("dev_cred_render_{}", std::process::id()));
    let config = CertificateConfig {
        project_root: ".".to_string(),
        ..Default::default()
    };

    let member = Member {
        id: "RVD-DEV-0001-TEST".to_string(),
        name: "Unit Test Recipient".to_string(),
        date: Some("2026-11-22".to_string()),
        legality_id: Some("RVD-65:A1B2-C3D4-E5F6".to_string()),
        member_no: Some(1),
        algorithm: Some("NIST FIPS 204 ML-DSA-65".to_string()),
        signature_sha3_256: Some("DUMMY".to_string()),
        status: Some("AUTHENTIC & VERIFIED".to_string()),
    };

    let files = render_certificate(
        &member,
        &config,
        ThemeMode::White,
        &temp_out,
        OutputFormat::All,
    )
    .unwrap();

    assert_eq!(files.len(), 3);
    for file in &files {
        assert!(file.exists());
        let meta = fs::metadata(file).unwrap();
        assert!(meta.len() > 0);
    }

    let _ = fs::remove_dir_all(&temp_out);
}
