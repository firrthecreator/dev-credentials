use crate::cli::format::OutputFormat;
use crate::cli::theme::ThemeMode;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "dev-credentials")]
#[command(about = "dev : Clean Minimalist Certificate Generator with Rivide ML-DSA-65 PQC")]
pub struct CliArgs {
    #[arg(long, default_value_t = false)]
    pub all: bool,

    #[arg(long)]
    pub name: Option<String>,

    #[arg(long, default_value = "DEV-2026-0008")]
    pub id: String,

    #[arg(long, default_value = "2026-11-22")]
    pub date: String,

    #[arg(long)]
    pub legality: Option<String>,

    #[arg(long, value_enum, default_value_t = ThemeMode::White)]
    pub theme: ThemeMode,

    #[arg(long, value_enum, default_value_t = OutputFormat::All)]
    pub format: OutputFormat,

    #[arg(long, default_value = "output")]
    pub output: String,

    #[arg(long, value_name = "CERT_ID")]
    pub verify: Option<String>,

    #[arg(long, default_value_t = false)]
    pub sign: bool,
}
