use crate::cli::theme::ThemeMode;
use cairo::{Context, ImageSurface};
use std::fs::File;
use std::path::Path;

pub struct LogoRenderer;

impl LogoRenderer {
    pub fn draw<P: AsRef<Path>>(
        ctx: &Context,
        project_root: P,
        theme_mode: ThemeMode,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let logo_filename = match theme_mode {
            ThemeMode::White => "logo_dark.png",
            ThemeMode::Black => "logo_light.png",
        };

        let nested_path = project_root
            .as_ref()
            .join("assets")
            .join("logos")
            .join(logo_filename);

        let logo_path = if nested_path.exists() {
            nested_path
        } else {
            project_root.as_ref().join("assets").join(logo_filename)
        };

        if !logo_path.exists() {
            return Ok(());
        }

        let mut file = File::open(&logo_path)?;
        let surface = match ImageSurface::create_from_png(&mut file) {
            Ok(s) => s,
            Err(_) => return Ok(()),
        };

        let orig_h = surface.height() as f64;
        if orig_h <= 0.0 {
            return Ok(());
        }

        let target_h = 72.0;
        let scale = target_h / orig_h;
        let logo_x = 75.0;
        let logo_y = 65.0;

        ctx.save()?;
        ctx.translate(logo_x, logo_y);
        ctx.scale(scale, scale);
        ctx.set_source_surface(&surface, 0.0, 0.0)?;
        ctx.paint()?;
        ctx.restore()?;

        Ok(())
    }
}
