pub mod components;
pub mod exporters;
pub mod layout;

pub use components::{
    BorderRenderer, ColophonRenderer, FooterRenderer, HeaderRenderer, LogoRenderer,
    RecipientRenderer, SignatoriesRenderer,
};
pub use exporters::{PdfRenderer, PngRenderer, SvgRenderer};
pub use layout::{CanvasGeometry, TextRenderer};

use crate::cli::format::OutputFormat;
use crate::cli::theme::ThemeMode;
use crate::domain::config::CertificateConfig;
use crate::domain::member::Member;
use crate::storage::sanitizer::Sanitizer;
use cairo::Context;
use std::fs;
use std::path::{Path, PathBuf};

pub fn draw_certificate(
    ctx: &Context,
    member: &Member,
    config: &CertificateConfig,
    theme_mode: ThemeMode,
) -> Result<(), Box<dyn std::error::Error>> {
    let palette = theme_mode.palette();

    ctx.set_source_rgb(palette.bg.r, palette.bg.g, palette.bg.b);
    ctx.paint()?;

    BorderRenderer::draw(ctx, &palette)?;

    LogoRenderer::draw(ctx, &config.project_root, theme_mode)?;

    HeaderRenderer::draw(ctx, config, &palette)?;

    RecipientRenderer::draw(ctx, member, config, &palette)?;

    SignatoriesRenderer::draw(ctx, config, &palette)?;

    FooterRenderer::draw(ctx, member, config, &palette)?;

    ColophonRenderer::draw(ctx, &palette)?;

    Ok(())
}

pub fn render_certificate<P: AsRef<Path>>(
    member: &Member,
    config: &CertificateConfig,
    theme_mode: ThemeMode,
    output_dir: P,
    format: OutputFormat,
) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
    let out_dir = output_dir.as_ref();
    if !out_dir.exists() {
        fs::create_dir_all(out_dir)?;
    }

    let clean_name = Sanitizer::sanitize_filename(&member.name);
    let cert_id = Sanitizer::sanitize_id(if member.id.is_empty() {
        "cert"
    } else {
        &member.id
    });
    let base_filename = format!("{}_{}", clean_name, cert_id);

    let mut generated_files = Vec::new();

    if format.includes_pdf() {
        let pdf_dir = out_dir.join("pdf");
        if !pdf_dir.exists() {
            fs::create_dir_all(&pdf_dir)?;
        }
        let pdf_file = pdf_dir.join(format!("{}.pdf", base_filename));
        PdfRenderer::render(member, config, theme_mode, &pdf_file)?;
        generated_files.push(pdf_file);
    }

    if format.includes_png() {
        let png_dir = out_dir.join("png");
        if !png_dir.exists() {
            fs::create_dir_all(&png_dir)?;
        }
        let png_file = png_dir.join(format!("{}.png", base_filename));
        PngRenderer::render(member, config, theme_mode, &png_file)?;
        generated_files.push(png_file);
    }

    if format.includes_svg() {
        let svg_dir = out_dir.join("svg");
        if !svg_dir.exists() {
            fs::create_dir_all(&svg_dir)?;
        }
        let svg_file = svg_dir.join(format!("{}.svg", base_filename));
        SvgRenderer::render(member, config, theme_mode, &svg_file)?;
        generated_files.push(svg_file);
    }

    Ok(generated_files)
}
