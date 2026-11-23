use crate::cli::args::CliArgs;
use crate::domain::config::CertificateConfig;
use crate::renderer::render_certificate;
use crate::storage::member_store::MemberStore;
use crate::storage::registry_store::RegistryStore;
use std::path::Path;

pub struct BatchRunner;

impl BatchRunner {
    pub fn run<P: AsRef<Path>>(
        root: P,
        config: &CertificateConfig,
        args: &CliArgs,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut members = MemberStore::load(&root)?;
        let registry = RegistryStore::load(&root)?;

        println!("Generating member certificates");
        println!("Theme: {:?} | Format: {:?}", args.theme, args.format);
        println!("Total members: {}\n", members.len());

        let mut total_generated = 0;
        let total = members.len();

        for (idx, member) in members.iter_mut().enumerate() {
            let cid = if member.id.is_empty() {
                format!("DEV-{}", idx + 1)
            } else {
                member.id.clone()
            };

            if let Some(record) = registry.get(&cid) {
                member.legality_id = Some(record.legality_id.clone());
            }

            println!(
                "[{}/{}] Generating: {} ({})",
                idx + 1,
                total,
                member.name,
                cid
            );
            println!(
                "  Legality ID: {}",
                member.legality_id.as_deref().unwrap_or("None")
            );

            let out_dir = root.as_ref().join(&args.output);
            let files = render_certificate(member, config, args.theme, &out_dir, args.format)?;

            for f in &files {
                println!("  Generated: {}", f.display());
            }
            total_generated += files.len();
        }

        println!(
            "\nCompleted. Generated {} certificate files in '{}/'.",
            total_generated, args.output
        );

        Ok(())
    }
}
