use crate::cli::theme::ThemePalette;
use crate::domain::config::CertificateConfig;
use crate::domain::member::Member;
use crate::renderer::layout::canvas::CanvasGeometry;
use crate::renderer::layout::text::TextRenderer;
use cairo::{Context, FontSlant, FontWeight};

pub struct RecipientRenderer;

impl RecipientRenderer {
    pub fn draw(
        ctx: &Context,
        member: &Member,
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
            &member.name,
            CanvasGeometry::BASE_W / 2.0,
            385.0,
            58.0,
            "Liberation Serif",
            FontWeight::Bold,
            FontSlant::Italic,
        )?;

        ctx.set_source_rgb(
            palette.text_secondary.r,
            palette.text_secondary.g,
            palette.text_secondary.b,
        );
        ctx.select_font_face("Inter", FontSlant::Normal, FontWeight::Normal);
        ctx.set_font_size(16.0);

        let lines = TextRenderer::wrap_text(ctx, &config.default_citation, 940.0)?;
        let mut line_y = 475.0;
        for line in lines {
            TextRenderer::draw_text_centered(
                ctx,
                &line,
                CanvasGeometry::BASE_W / 2.0,
                line_y,
                16.0,
                "Inter",
                FontWeight::Normal,
                FontSlant::Normal,
            )?;
            line_y += 30.0;
        }

        Ok(())
    }
}
