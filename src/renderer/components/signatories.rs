use crate::cli::theme::ThemePalette;
use crate::domain::config::CertificateConfig;
use crate::renderer::layout::canvas::CanvasGeometry;
use crate::renderer::layout::text::TextRenderer;
use cairo::{Context, FontSlant, FontWeight};

pub struct SignatoriesRenderer;

impl SignatoriesRenderer {
    pub fn draw(
        ctx: &Context,
        config: &CertificateConfig,
        palette: &ThemePalette,
    ) -> Result<(), cairo::Error> {
        let signers = &config.signatories;
        let sig_y = 665.0;
        let count = signers.len();

        let sig_x_positions: Vec<f64> = match count {
            1 => vec![CanvasGeometry::BASE_W / 2.0],
            2 => vec![CanvasGeometry::BASE_W * 0.35, CanvasGeometry::BASE_W * 0.65],
            3 => vec![300.0, 700.0, 1100.0],
            _ => {
                let spacing = (CanvasGeometry::BASE_W - 300.0) / ((count + 1) as f64);
                (0..count)
                    .map(|i| 150.0 + spacing * ((i + 1) as f64))
                    .collect()
            }
        };

        for (idx, signer) in signers.iter().enumerate() {
            let cx = sig_x_positions[idx];

            ctx.save()?;
            ctx.set_source_rgb(
                palette.sig_line.r,
                palette.sig_line.g,
                palette.sig_line.b,
            );
            ctx.set_line_width(0.8);
            ctx.move_to(cx - 95.0, sig_y);
            ctx.line_to(cx + 95.0, sig_y);
            ctx.stroke()?;
            ctx.restore()?;

            ctx.set_source_rgb(
                palette.text_primary.r,
                palette.text_primary.g,
                palette.text_primary.b,
            );
            TextRenderer::draw_text_centered(
                ctx,
                &signer.name,
                cx,
                sig_y + 28.0,
                16.0,
                "Inter",
                FontWeight::Bold,
                FontSlant::Normal,
            )?;

            ctx.set_source_rgb(
                palette.text_muted.r,
                palette.text_muted.g,
                palette.text_muted.b,
            );
            let role_words: Vec<String> = signer
                .role
                .to_uppercase()
                .split_whitespace()
                .map(|s| s.to_string())
                .collect();
            let role_spaced = role_words.join("   ");

            TextRenderer::draw_text_centered(
                ctx,
                &role_spaced,
                cx,
                sig_y + 50.0,
                9.5,
                "Inter",
                FontWeight::Bold,
                FontSlant::Normal,
            )?;
        }

        Ok(())
    }
}
