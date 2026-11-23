use clap::Parser;
use dev_credentials::app::{BatchRunner, SignRunner, SingleRunner, VerifyRunner};
use dev_credentials::cli::CliArgs;
use dev_credentials::storage::ConfigStore;
use std::env;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = CliArgs::parse();
    let current_dir: PathBuf = env::current_dir()?;

    if let Some(ref verify_id) = args.verify {
        VerifyRunner::run(&current_dir, verify_id)?;
        return Ok(());
    }

    if args.sign {
        SignRunner::run(&current_dir)?;
        return Ok(());
    }

    let config = ConfigStore::load(&current_dir)?;

    if let Some(ref name) = args.name {
        SingleRunner::run(&current_dir, &config, &args, name)?;
    } else {
        BatchRunner::run(&current_dir, &config, &args)?;
    }

    Ok(())
}
