use crate::domain::signatory::Signatory;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CertificateConfig {
    #[serde(default = "default_circle_name")]
    pub circle_name: String,
    #[serde(default = "default_circle_tagline")]
    pub circle_tagline: String,
    #[serde(default = "default_certificate_title")]
    pub default_certificate_title: String,
    #[serde(default = "default_subtitle")]
    pub default_subtitle: String,
    #[serde(default = "default_citation")]
    pub default_citation: String,
    #[serde(default = "default_issue_date")]
    pub issue_date: String,
    #[serde(default = "default_status")]
    pub status: String,
    #[serde(default = "default_verify_url_base")]
    pub verify_url_base: String,
    #[serde(default = "default_signatories")]
    pub signatories: Vec<Signatory>,
    #[serde(default)]
    pub project_root: String,
}

fn default_circle_name() -> String {
    "dev :".to_string()
}

fn default_circle_tagline() -> String {
    "OFFICIAL COMMUNITY".to_string()
}

fn default_certificate_title() -> String {
    "CERTIFICATE OF MEMBERSHIP".to_string()
}

fn default_subtitle() -> String {
    "THIS IS PROUDLY PRESENTED TO".to_string()
}

fn default_citation() -> String {
    "In recognition of exceptional dedication, trusted fellowship, and invaluable contributions to the dev : community.".to_string()
}

fn default_issue_date() -> String {
    "2026-11-22".to_string()
}

fn default_status() -> String {
    "OFFICIAL & VERIFIED".to_string()
}

fn default_verify_url_base() -> String {
    "https://mrvlous.github.io/dev/verify.html".to_string()
}

fn default_signatories() -> Vec<Signatory> {
    vec![
        Signatory {
            name: "Moh. Ananda Firmansyah Putra".to_string(),
            role: "Founder & Owner".to_string(),
        },
        Signatory {
            name: "Adnan Choirul Ramadhani".to_string(),
            role: "Co-Founder & Admin".to_string(),
        },
        Signatory {
            name: "Rifky Kurniawan".to_string(),
            role: "Co-Founder & Admin".to_string(),
        },
    ]
}

impl Default for CertificateConfig {
    fn default() -> Self {
        Self {
            circle_name: default_circle_name(),
            circle_tagline: default_circle_tagline(),
            default_certificate_title: default_certificate_title(),
            default_subtitle: default_subtitle(),
            default_citation: default_citation(),
            issue_date: default_issue_date(),
            status: default_status(),
            verify_url_base: default_verify_url_base(),
            signatories: default_signatories(),
            project_root: ".".to_string(),
        }
    }
}
