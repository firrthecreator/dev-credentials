use crate::domain::member::Member;
use crate::storage::path_resolver::PathResolver;
use std::fs;
use std::io::Result;
use std::path::Path;

pub struct MemberStore;

impl MemberStore {
    pub fn load<P: AsRef<Path>>(root: P) -> Result<Vec<Member>> {
        let path = PathResolver::members_file(root);
        let data = fs::read_to_string(path)?;
        let members: Vec<Member> = serde_json::from_str(&data)?;
        Ok(members)
    }

    pub fn save<P: AsRef<Path>>(root: P, members: &[Member]) -> Result<()> {
        let path = PathResolver::members_file(root);
        if let Some(parent) = path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent)?;
            }
        }
        let data = serde_json::to_string_pretty(members)?;
        fs::write(path, data)
    }
}
