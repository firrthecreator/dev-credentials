use crate::cli::theme::ThemePalette;
use crate::domain::config::CertificateConfig;
use crate::domain::member::Member;
use crate::renderer::layout::canvas::CanvasGeometry;
use cairo::{Context, FontSlant, FontWeight};

pub struct FooterRenderer;

impl FooterRenderer {
    pub const FOOT_TOP_Y: f64 = 852.0;

    pub fn draw(
        ctx: &Context,
        member: &Member,
        config: &CertificateConfig,
        palette: &ThemePalette,
    ) -> Result<(), cairo::Error> {
        ctx.save()?;
        ctx.set_source_rgb(
            palette.border_hairline.r,
            palette.border_hairline.g,
            palette.border_hairline.b,
        );
        ctx.set_line_width(0.8);
        ctx.move_to(70.0, Self::FOOT_TOP_Y);
        ctx.line_to(CanvasGeometry::BASE_W - 70.0, Self::FOOT_TOP_Y);
        ctx.stroke()?;
        ctx.restore()?;

        let issue_date = member
            .date
            .as_deref()
            .unwrap_or(&config.issue_date);
        let cert_id = if member.id.is_empty() {
            "DEV-2026-0000"
        } else {
            &member.id
        };
        let legality_id = member
            .legality_id
            .as_deref()
            .unwrap_or("RVD-65:PENDING");

        Self::draw_meta_cell(ctx, palette, "Date of Issuance", issue_date, 70.0)?;
        Self::draw_v_divider(ctx, palette, 450.0)?;

        Self::draw_meta_cell(ctx, palette, "Credential ID", cert_id, 510.0)?;
        Self::draw_v_divider(ctx, palette, 940.0)?;

        Self::draw_meta_cell(ctx, palette, "Legality ID", legality_id, 1000.0)?;

        Ok(())
    }

    fn draw_meta_cell(
        ctx: &Context,
        palette: &ThemePalette,
        label: &str,
        val: &str,
        x: f64,
    ) -> Result<(), cairo::Error> {
        ctx.save()?;
        ctx.set_source_rgb(
            palette.text_muted.r,
            palette.text_muted.g,
            palette.text_muted.b,
        );
        ctx.select_font_face("Inter", FontSlant::Normal, FontWeight::Bold);
        ctx.set_font_size(9.0);
        ctx.move_to(x, Self::FOOT_TOP_Y + 25.0);
        ctx.show_text(&label.to_uppercase())?;

        ctx.set_source_rgb(
            palette.text_primary.r,
            palette.text_primary.g,
            palette.text_primary.b,
        );
        ctx.select_font_face("JetBrains Mono", FontSlant::Normal, FontWeight::Bold);
        ctx.set_font_size(12.0);
        ctx.move_to(x, Self::FOOT_TOP_Y + 46.0);
        ctx.show_text(val)?;
        ctx.restore()?;
        Ok(())
    }

    fn draw_v_divider(
        ctx: &Context,
        palette: &ThemePalette,
        vx: f64,
    ) -> Result<(), cairo::Error> {
        ctx.save()?;
        ctx.set_source_rgb(
            palette.border_hairline.r,
            palette.border_hairline.g,
            palette.border_hairline.b,
        );
        ctx.set_line_width(0.8);
        ctx.move_to(vx, Self::FOOT_TOP_Y + 12.0);
        ctx.line_to(vx, Self::FOOT_TOP_Y + 54.0);
        ctx.stroke()?;
        ctx.restore()?;
        Ok(())
    }
}
