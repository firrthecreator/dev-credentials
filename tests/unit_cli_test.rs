use dev_credentials::cli::{OutputFormat, ThemeMode};

#[test]
fn test_theme_mode_palette_white() {
    let palette = ThemeMode::White.palette();
    assert_eq!(palette.bg.r, 1.0);
    assert_eq!(palette.bg.g, 1.0);
    assert_eq!(palette.bg.b, 1.0);
    assert_eq!(palette.border_main.r, 0.06);
}

#[test]
fn test_theme_mode_palette_black() {
    let palette = ThemeMode::Black.palette();
    assert_eq!(palette.bg.r, 0.04);
    assert_eq!(palette.bg.g, 0.04);
    assert_eq!(palette.bg.b, 0.04);
    assert_eq!(palette.border_main.r, 0.95);
}

#[test]
fn test_output_format_flags() {
    assert!(OutputFormat::All.includes_pdf());
    assert!(OutputFormat::All.includes_png());
    assert!(OutputFormat::All.includes_svg());

    assert!(OutputFormat::Pdf.includes_pdf());
    assert!(!OutputFormat::Pdf.includes_png());
    assert!(!OutputFormat::Pdf.includes_svg());

    assert!(!OutputFormat::Png.includes_pdf());
    assert!(OutputFormat::Png.includes_png());
    assert!(!OutputFormat::Png.includes_svg());

    assert!(!OutputFormat::Svg.includes_pdf());
    assert!(!OutputFormat::Svg.includes_png());
    assert!(OutputFormat::Svg.includes_svg());
}
