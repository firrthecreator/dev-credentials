use cairo::{Context, FontSlant, FontWeight, TextExtents};

pub struct TextRenderer;

impl TextRenderer {
    pub fn draw_text_centered(
        ctx: &Context,
        text: &str,
        cx: f64,
        y: f64,
        font_size: f64,
        font_family: &str,
        font_weight: FontWeight,
        font_slant: FontSlant,
    ) -> Result<TextExtents, cairo::Error> {
        ctx.select_font_face(font_family, font_slant, font_weight);
        ctx.set_font_size(font_size);
        let extents = ctx.text_extents(text)?;
        let x = cx - (extents.width() / 2.0 + extents.x_bearing());
        ctx.move_to(x, y);
        ctx.show_text(text)?;
        Ok(extents)
    }

    pub fn wrap_text(ctx: &Context, text: &str, max_width: f64) -> Result<Vec<String>, cairo::Error> {
        let words: Vec<&str> = text.split_whitespace().collect();
        let mut lines: Vec<String> = Vec::new();
        let mut current_line: Vec<&str> = Vec::new();

        for word in words {
            let mut test_words = current_line.clone();
            test_words.push(word);
            let test_line = test_words.join(" ");
            let extents = ctx.text_extents(&test_line)?;

            if extents.width() <= max_width {
                current_line.push(word);
            } else if !current_line.is_empty() {
                lines.push(current_line.join(" "));
                current_line = vec![word];
            } else {
                lines.push(word.to_string());
                current_line = Vec::new();
            }
        }

        if !current_line.is_empty() {
            lines.push(current_line.join(" "));
        }

        Ok(lines)
    }
}
