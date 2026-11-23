use crate::cli::theme::ThemePalette;
use crate::renderer::layout::canvas::CanvasGeometry;
use cairo::Context;

pub struct BorderRenderer;

impl BorderRenderer {
    pub fn draw(ctx: &Context, palette: &ThemePalette) -> Result<(), cairo::Error> {
        ctx.set_source_rgb(
            palette.border_hairline.r,
            palette.border_hairline.g,
            palette.border_hairline.b,
        );
        ctx.set_line_width(0.8);
        ctx.rectangle(
            36.0,
            36.0,
            CanvasGeometry::BASE_W - 72.0,
            CanvasGeometry::BASE_H - 72.0,
        );
        ctx.stroke()?;

        ctx.set_source_rgb(
            palette.border_main.r,
            palette.border_main.g,
            palette.border_main.b,
        );
        ctx.set_line_width(1.8);
        ctx.rectangle(
            48.0,
            48.0,
            CanvasGeometry::BASE_W - 96.0,
            CanvasGeometry::BASE_H - 96.0,
        );
        ctx.stroke()?;

        Self::draw_corner(ctx, palette, 48.0, 48.0, 1.0, 1.0)?;
        Self::draw_corner(
            ctx,
            palette,
            CanvasGeometry::BASE_W - 48.0,
            48.0,
            -1.0,
            1.0,
        )?;
        Self::draw_corner(
            ctx,
            palette,
            48.0,
            CanvasGeometry::BASE_H - 48.0,
            1.0,
            -1.0,
        )?;
        Self::draw_corner(
            ctx,
            palette,
            CanvasGeometry::BASE_W - 48.0,
            CanvasGeometry::BASE_H - 48.0,
            -1.0,
            -1.0,
        )?;

        Ok(())
    }

    fn draw_corner(
        ctx: &Context,
        palette: &ThemePalette,
        cx: f64,
        cy: f64,
        dx: f64,
        dy: f64,
    ) -> Result<(), cairo::Error> {
        ctx.save()?;
        ctx.set_source_rgb(
            palette.border_main.r,
            palette.border_main.g,
            palette.border_main.b,
        );
        ctx.set_line_width(1.4);
        ctx.move_to(cx, cy + dy * 14.0);
        ctx.line_to(cx, cy);
        ctx.line_to(cx + dx * 14.0, cy);
        ctx.stroke()?;
        ctx.restore()?;
        Ok(())
    }
}
