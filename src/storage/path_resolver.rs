use std::path::{Path, PathBuf};

pub struct PathResolver;

impl PathResolver {
    pub fn data_dir<P: AsRef<Path>>(root: P) -> PathBuf {
        root.as_ref().join("data")
    }

    pub fn config_file<P: AsRef<Path>>(root: P) -> PathBuf {
        let in_data = Self::data_dir(&root).join("config.json");
        if in_data.exists() {
            in_data
        } else {
            let in_root = root.as_ref().join("config.json");
            if in_root.exists() {
                in_root
            } else {
                in_data
            }
        }
    }

    pub fn members_file<P: AsRef<Path>>(root: P) -> PathBuf {
        let in_data = Self::data_dir(&root).join("members.json");
        if in_data.exists() {
            in_data
        } else {
            let in_root = root.as_ref().join("members.json");
            if in_root.exists() {
                in_root
            } else {
                in_data
            }
        }
    }

    pub fn registry_file<P: AsRef<Path>>(root: P) -> PathBuf {
        let in_data = Self::data_dir(&root).join("db_registry.json");
        if in_data.exists() {
            in_data
        } else {
            let in_root = root.as_ref().join("db_registry.json");
            if in_root.exists() {
                in_root
            } else {
                in_data
            }
        }
    }

    pub fn keys_dir<P: AsRef<Path>>(root: P) -> PathBuf {
        root.as_ref().join("keys").join("authority")
    }

    pub fn keypair_file<P: AsRef<Path>>(root: P) -> PathBuf {
        let nested = Self::keys_dir(&root).join("authority_mldsa65.json");
        if nested.exists() {
            nested
        } else {
            let legacy = root.as_ref().join("keys").join("authority_mldsa65.json");
            if legacy.exists() {
                legacy
            } else {
                nested
            }
        }
    }

    pub fn assets_dir<P: AsRef<Path>>(root: P) -> PathBuf {
        root.as_ref().join("assets")
    }

    pub fn logos_dir<P: AsRef<Path>>(root: P) -> PathBuf {
        root.as_ref().join("assets").join("logos")
    }

    pub fn output_dir<P: AsRef<Path>>(base: P) -> PathBuf {
        base.as_ref().to_path_buf()
    }
}
