use crate::cli::theme::ThemePalette;
use crate::renderer::components::footer::FooterRenderer;
use crate::renderer::layout::canvas::CanvasGeometry;
use crate::renderer::layout::text::TextRenderer;
use cairo::{Context, FontSlant, FontWeight};

pub struct ColophonRenderer;

impl ColophonRenderer {
    pub fn draw(ctx: &Context, palette: &ThemePalette) -> Result<(), cairo::Error> {
        ctx.save()?;
        ctx.set_source_rgb(
            palette.text_muted.r,
            palette.text_muted.g,
            palette.text_muted.b,
        );
        TextRenderer::draw_text_centered(
            ctx,
            "CRYPTOGRAPHIC INTEGRITY POWERED BY RIVIDE",
            CanvasGeometry::BASE_W / 2.0,
            FooterRenderer::FOOT_TOP_Y + 70.0,
            8.0,
            "JetBrains Mono",
            FontWeight::Normal,
            FontSlant::Normal,
        )?;
        ctx.restore()?;
        Ok(())
    }
}
