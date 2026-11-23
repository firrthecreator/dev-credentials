use crate::domain::record::CertificateRecord;
use crate::storage::path_resolver::PathResolver;
use std::collections::BTreeMap;
use std::fs;
use std::io::Result;
use std::path::Path;

pub struct RegistryStore;

impl RegistryStore {
    pub fn load<P: AsRef<Path>>(root: P) -> Result<BTreeMap<String, CertificateRecord>> {
        let path = PathResolver::registry_file(root);
        if path.exists() {
            let data = fs::read_to_string(path)?;
            let reg: BTreeMap<String, CertificateRecord> = serde_json::from_str(&data)?;
            Ok(reg)
        } else {
            Ok(BTreeMap::new())
        }
    }

    pub fn save<P: AsRef<Path>>(
        root: P,
        registry: &BTreeMap<String, CertificateRecord>,
    ) -> Result<()> {
        let path = PathResolver::registry_file(root);
        if let Some(parent) = path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent)?;
            }
        }
        let data = serde_json::to_string_pretty(registry)?;
        fs::write(path, data)
    }
}
