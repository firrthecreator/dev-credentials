use clap::ValueEnum;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RgbColor {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl RgbColor {
    pub const fn new(r: f64, g: f64, b: f64) -> Self {
        Self { r, g, b }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ThemePalette {
    pub bg: RgbColor,
    pub border_main: RgbColor,
    pub border_hairline: RgbColor,
    pub text_primary: RgbColor,
    pub text_secondary: RgbColor,
    pub text_muted: RgbColor,
    pub sig_line: RgbColor,
    pub sig_ink: RgbColor,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, ValueEnum, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ThemeMode {
    White,
    Black,
}

impl ThemeMode {
    pub fn palette(&self) -> ThemePalette {
        match self {
            Self::White => ThemePalette {
                bg: RgbColor::new(1.0, 1.0, 1.0),
                border_main: RgbColor::new(0.06, 0.06, 0.06),
                border_hairline: RgbColor::new(0.85, 0.85, 0.85),
                text_primary: RgbColor::new(0.03, 0.03, 0.03),
                text_secondary: RgbColor::new(0.24, 0.26, 0.30),
                text_muted: RgbColor::new(0.45, 0.48, 0.52),
                sig_line: RgbColor::new(0.78, 0.80, 0.84),
                sig_ink: RgbColor::new(0.04, 0.04, 0.04),
            },
            Self::Black => ThemePalette {
                bg: RgbColor::new(0.04, 0.04, 0.04),
                border_main: RgbColor::new(0.95, 0.95, 0.95),
                border_hairline: RgbColor::new(0.25, 0.25, 0.25),
                text_primary: RgbColor::new(0.98, 0.98, 0.98),
                text_secondary: RgbColor::new(0.78, 0.80, 0.84),
                text_muted: RgbColor::new(0.52, 0.55, 0.60),
                sig_line: RgbColor::new(0.35, 0.35, 0.35),
                sig_ink: RgbColor::new(0.95, 0.95, 0.95),
            },
        }
    }
}
