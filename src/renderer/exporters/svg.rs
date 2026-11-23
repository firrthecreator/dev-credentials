use crate::cli::theme::ThemeMode;
use crate::domain::config::CertificateConfig;
use crate::domain::member::Member;
use crate::renderer::draw_certificate;
use crate::renderer::layout::canvas::CanvasGeometry;
use cairo::{Context, SvgSurface};
use std::path::Path;

pub struct SvgRenderer;

impl SvgRenderer {
    pub fn render<P: AsRef<Path>>(
        member: &Member,
        config: &CertificateConfig,
        theme_mode: ThemeMode,
        output_file: P,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let surface = SvgSurface::new(
            CanvasGeometry::BASE_W,
            CanvasGeometry::BASE_H,
            Some(output_file.as_ref()),
        )?;
        let ctx = Context::new(&surface)?;
        draw_certificate(&ctx, member, config, theme_mode)?;
        surface.finish();
        Ok(())
    }
}
