use clap::ValueEnum;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, ValueEnum, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OutputFormat {
    Pdf,
    Png,
    Svg,
    All,
}

impl OutputFormat {
    pub fn includes_pdf(&self) -> bool {
        matches!(self, Self::Pdf | Self::All)
    }

    pub fn includes_png(&self) -> bool {
        matches!(self, Self::Png | Self::All)
    }

    pub fn includes_svg(&self) -> bool {
        matches!(self, Self::Svg | Self::All)
    }
}
