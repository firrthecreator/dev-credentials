use crate::cli::theme::ThemeMode;
use crate::domain::config::CertificateConfig;
use crate::domain::member::Member;
use crate::renderer::draw_certificate;
use crate::renderer::layout::canvas::CanvasGeometry;
use cairo::{Context, Format, ImageSurface};
use std::fs::File;
use std::path::Path;

pub struct PngRenderer;

impl PngRenderer {
    pub fn render<P: AsRef<Path>>(
        member: &Member,
        config: &CertificateConfig,
        theme_mode: ThemeMode,
        output_file: P,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let surface = ImageSurface::create(
            Format::ARgb32,
            CanvasGeometry::PNG_W,
            CanvasGeometry::PNG_H,
        )?;
        let ctx = Context::new(&surface)?;
        ctx.scale(
            (CanvasGeometry::PNG_W as f64) / CanvasGeometry::BASE_W,
            (CanvasGeometry::PNG_H as f64) / CanvasGeometry::BASE_H,
        );
        draw_certificate(&ctx, member, config, theme_mode)?;

        let mut file = File::create(output_file)?;
        surface.write_to_png(&mut file)?;
        Ok(())
    }
}
