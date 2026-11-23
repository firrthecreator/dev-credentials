use crate::domain::config::CertificateConfig;
use crate::storage::path_resolver::PathResolver;
use std::fs;
use std::io::Result;
use std::path::Path;

pub struct ConfigStore;

impl ConfigStore {
    pub fn load<P: AsRef<Path>>(root: P) -> Result<CertificateConfig> {
        let path = PathResolver::config_file(&root);
        if path.exists() {
            let data = fs::read_to_string(path)?;
            let mut cfg: CertificateConfig = serde_json::from_str(&data)?;
            cfg.project_root = root.as_ref().to_string_lossy().to_string();
            Ok(cfg)
        } else {
            let mut cfg = CertificateConfig::default();
            cfg.project_root = root.as_ref().to_string_lossy().to_string();
            Ok(cfg)
        }
    }

    pub fn save<P: AsRef<Path>>(root: P, config: &CertificateConfig) -> Result<()> {
        let path = PathResolver::config_file(root);
        if let Some(parent) = path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent)?;
            }
        }
        let data = serde_json::to_string_pretty(config)?;
        fs::write(path, data)
    }
}
