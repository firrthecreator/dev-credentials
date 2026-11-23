use crate::crypto::keypair::{AuthorityKeypair, AuthorityKeypairDto};
use crate::storage::path_resolver::PathResolver;
use std::fs;
use std::path::Path;

pub struct KeyStore;

impl KeyStore {
    pub fn load_or_create<P: AsRef<Path>>(
        root: P,
    ) -> std::result::Result<AuthorityKeypair, Box<dyn std::error::Error>> {
        let keys_dir = PathResolver::keys_dir(&root);
        if !keys_dir.exists() {
            fs::create_dir_all(&keys_dir)?;
        }

        let key_path = PathResolver::keypair_file(&root);
        if key_path.exists() {
            let data = fs::read_to_string(&key_path)?;
            let dto: AuthorityKeypairDto = serde_json::from_str(&data)?;
            AuthorityKeypair::from_dto(&dto)
        } else {
            let kp = AuthorityKeypair::generate()?;
            let dto = kp.to_dto();
            let data = serde_json::to_string_pretty(&dto)?;
            fs::write(&key_path, data)?;
            Ok(kp)
        }
    }
}
