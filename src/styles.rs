use crate::document::Style;
#[allow(dead_code)]
/// Trait for styling document elements
pub trait Stylable {
    fn apply_style(&self, style: &Style) -> Self;
}

impl Stylable for String {
    fn apply_style(&self, style: &Style) -> Self {
        let mut styled = self.clone();
        if style.bold {
            styled = format!("**{}**", styled);
        }
        if style.italic {
            styled = format!("*{}*", styled);
        }
        if let Some(ref color) = style.color {
            styled = format!("<span style=\"color:{}\">{}</span>", color, styled);
        }
        if let Some(ref font) = style.font {
            styled = format!("<span style=\"font-family:{}\">{}</span>", font, styled);
        }
        if let Some(size) = style.font_size {
            styled = format!("<span style=\"font-size:{}pt\">{}</span>", size, styled);
        }
        styled
    }
}
//more feature TODO;
