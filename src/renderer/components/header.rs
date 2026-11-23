use crate::cli::theme::ThemePalette;
use crate::domain::config::CertificateConfig;
use crate::renderer::layout::canvas::CanvasGeometry;
use crate::renderer::layout::text::TextRenderer;
use cairo::{Context, FontSlant, FontWeight};

pub struct HeaderRenderer;

impl HeaderRenderer {
    pub fn draw(
        ctx: &Context,
        config: &CertificateConfig,
        palette: &ThemePalette,
    ) -> Result<(), cairo::Error> {
        ctx.set_source_rgb(
            palette.text_primary.r,
            palette.text_primary.g,
            palette.text_primary.b,
        );
        TextRenderer::draw_text_centered(
            ctx,
            &config.default_certificate_title,
            CanvasGeometry::BASE_W / 2.0,
            215.0,
            38.0,
            "Inter",
            FontWeight::Bold,
            FontSlant::Normal,
        )?;

        ctx.set_source_rgb(
            palette.text_muted.r,
            palette.text_muted.g,
            palette.text_muted.b,
        );
        let subtitle_spaced: String = config
            .default_subtitle
            .chars()
            .map(|c| c.to_string())
            .collect::<Vec<String>>()
            .join(" ");

        TextRenderer::draw_text_centered(
            ctx,
            &subtitle_spaced,
            CanvasGeometry::BASE_W / 2.0,
            265.0,
            11.0,
            "Inter",
            FontWeight::Bold,
            FontSlant::Normal,
        )?;

        Ok(())
    }
}
