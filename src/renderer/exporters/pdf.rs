use crate::cli::theme::ThemeMode;
use crate::domain::config::CertificateConfig;
use crate::domain::member::Member;
use crate::renderer::draw_certificate;
use crate::renderer::layout::canvas::CanvasGeometry;
use cairo::{Context, PdfSurface};
use std::path::Path;

pub struct PdfRenderer;

impl PdfRenderer {
    pub fn render<P: AsRef<Path>>(
        member: &Member,
        config: &CertificateConfig,
        theme_mode: ThemeMode,
        output_file: P,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let surface = PdfSurface::new(
            CanvasGeometry::A4_W,
            CanvasGeometry::A4_H,
            output_file.as_ref(),
        )?;
        let ctx = Context::new(&surface)?;
        ctx.scale(
            CanvasGeometry::A4_W / CanvasGeometry::BASE_W,
            CanvasGeometry::A4_H / CanvasGeometry::BASE_H,
        );
        draw_certificate(&ctx, member, config, theme_mode)?;
        surface.finish();
        Ok(())
    }
}
