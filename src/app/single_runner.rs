use crate::cli::args::CliArgs;
use crate::domain::config::CertificateConfig;
use crate::domain::member::Member;
use crate::renderer::render_certificate;
use crate::storage::registry_store::RegistryStore;
use rivide::crypto::Sha3;
use std::path::Path;

pub struct SingleRunner;

impl SingleRunner {
    pub fn run<P: AsRef<Path>>(
        root: P,
        config: &CertificateConfig,
        args: &CliArgs,
        name: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let registry = RegistryStore::load(&root)?;

        let mut member = Member {
            id: args.id.clone(),
            name: name.to_string(),
            date: Some(args.date.clone()),
            legality_id: None,
            member_no: None,
            algorithm: None,
            signature_sha3_256: None,
            status: None,
        };

        if let Some(record) = registry.get(&args.id) {
            member.legality_id = Some(record.legality_id.clone());
        } else if let Some(ref custom_legality) = args.legality {
            member.legality_id = Some(custom_legality.clone());
        } else {
            let seed_str = format!("{}:{}", args.id, name);
            let h_bytes = Sha3::sha3_256(seed_str.as_bytes());
            let h_hex = hex::encode(h_bytes).to_uppercase();
            member.legality_id = Some(format!(
                "RVD-65:{}-{}-{}",
                &h_hex[0..4],
                &h_hex[4..8],
                &h_hex[8..12]
            ));
        }

        println!("Generating certificate for {}", member.name);
        println!("Credential ID: {}", member.id);
        println!(
            "Legality ID: {}",
            member.legality_id.as_deref().unwrap_or("None")
        );

        let out_dir = root.as_ref().join(&args.output);
        let files = render_certificate(&member, config, args.theme, &out_dir, args.format)?;

        for f in &files {
            println!("Generated: {}", f.display());
        }
        println!("Completed successfully.");

        Ok(())
    }
}
