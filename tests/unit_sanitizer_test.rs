use dev_credentials::storage::Sanitizer;

#[test]
fn test_sanitize_filename_simple() {
    assert_eq!(
        Sanitizer::sanitize_filename("John Doe"),
        "john_doe"
    );
}

#[test]
fn test_sanitize_filename_special_chars() {
    assert_eq!(
        Sanitizer::sanitize_filename("Moh. Ananda Firmansyah Putra!"),
        "moh_ananda_firmansyah_putra"
    );
}

#[test]
fn test_sanitize_filename_multiple_symbols() {
    assert_eq!(
        Sanitizer::sanitize_filename("dev : community test"),
        "dev_community_test"
    );
}

#[test]
fn test_sanitize_filename_empty() {
    assert_eq!(Sanitizer::sanitize_filename(""), "credential");
    assert_eq!(Sanitizer::sanitize_filename("___"), "credential");
}

#[test]
fn test_sanitize_id() {
    assert_eq!(
        Sanitizer::sanitize_id("RVD:DEV/2026:0001"),
        "RVDDEV_20260001"
    );
}
