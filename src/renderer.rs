use crate::document::*;
use crate::errors::RustaTexError;
use log::debug;
use printpdf::*;
use std::fs::File;
use std::io::BufWriter;

pub fn render_pdf(
    document: &Document,
    output_path: &str,
    config: &RenderConfig,
) -> Result<(), RustaTexError> {
    debug!("Initializing PDF document.");
    let (doc, page1, layer1) = PdfDocument::new(
        "RustaTex Document",
        config.page_width,
        config.page_height,
        "Layer 1",
    );
    let mut current_layer = doc.get_page(page1).get_layer(layer1);

    let font = doc.add_builtin_font(BuiltinFont::TimesRoman).unwrap();

    let mut y_position = config.start_y;

    // Render metadata
    if let Some(title) = &document.title {
        current_layer.use_text(
            title,
            config.title_font_size,
            Mm(config.margin_left),
            Mm(y_position),
            &font,
        );
        debug!("Rendered title: {}", title);
        y_position -= config.title_font_size + 10.0;
    }
    if let Some(author) = &document.author {
        current_layer.use_text(
            format!("Author: {}", author),
            config.font_size,
            Mm(config.margin_left),
            Mm(y_position),
            &font,
        );
        debug!("Rendered author: {}", author);
        y_position -= config.font_size + 5.0;
    }
    if let Some(date) = &document.date {
        current_layer.use_text(
            format!("Date: {}", date),
            config.font_size,
            Mm(config.margin_left),
            Mm(y_position),
            &font,
        );
        debug!("Rendered date: {}", date);
        y_position -= config.font_size + 15.0;
    }

    // Render document elements
    for element in &document.elements {
        match element {
            DocumentElement::Section(sec) => {
                y_position -= config.section_spacing;
                current_layer.use_text(
                    &sec.title,
                    config.section_font_size,
                    Mm(config.margin_left),
                    Mm(y_position),
                    &font,
                );
                debug!("Rendered section: {}", sec.title);
                y_position -= config.section_font_size + config.paragraph_spacing;
            }
            DocumentElement::Subsection(subsec) => {
                y_position -= config.subsection_spacing;
                current_layer.use_text(
                    &subsec.title,
                    config.subsection_font_size,
                    Mm(config.margin_left + 10.0),
                    Mm(y_position),
                    &font,
                );
                debug!("Rendered subsection: {}", subsec.title);
                y_position -= config.subsection_font_size + config.paragraph_spacing;
            }
            DocumentElement::Paragraph(paragraph) => {
                let lines = wrap_text(
                    &paragraph.text,
                    config.line_width,
                    config.font_size as u32,
                    &font,
                );
                for line in lines {
                    current_layer.use_text(
                        &line,
                        config.font_size,
                        Mm(config.margin_left),
                        Mm(y_position),
                        &font,
                    );
                    y_position -= config.font_size + config.line_spacing;
                    debug!("Rendered paragraph line: {}", line);
                    if y_position < config.bottom_margin {
                        // Add new page if necessary
                        let (new_page, new_layer) =
                            doc.add_page(config.page_width, config.page_height, "Layer 1");
                        current_layer = doc.get_page(new_page).get_layer(new_layer);
                        y_position = config.start_y;
                        debug!("Added new page.");
                    }
                }
                y_position -= config.paragraph_spacing;
            }
        }
    }

    debug!("Saving PDF to {}", output_path);
    doc.save(&mut BufWriter::new(File::create(output_path)?))
        .map_err(|e| RustaTexError::RenderError(e.to_string()))?;
    Ok(())
}

fn wrap_text(text: &str, max_width_mm: f64, font_size: u32, font: &IndirectFontRef) -> Vec<String> {
    let words: Vec<&str> = text.split_whitespace().collect();
    let mut lines = Vec::new();
    let mut current_line = String::new();
    let mut current_width = 0.0;

    for word in words {
        let word_width = estimate_text_width(word, font_size, font);
        if current_width + word_width > max_width_mm {
            if !current_line.is_empty() {
                lines.push(current_line.clone());
                current_line.clear();
                current_width = 0.0;
            }
        }
        if !current_line.is_empty() {
            current_line.push(' ');
            current_width += estimate_text_width(" ", font_size, font);
        }
        current_line.push_str(word);
        current_width += word_width;
    }

    if !current_line.is_empty() {
        lines.push(current_line);
    }

    lines
}

fn estimate_text_width(text: &str, font_size: u32, _font: &IndirectFontRef) -> f64 {
    text.len() as f64 * (font_size as f64 * 0.5)
}
#[allow(dead_code)]
/// Configuration for rendering
pub struct RenderConfig {
    pub page_width: Mm,
    pub page_height: Mm,
    pub margin_left: f64,
    pub margin_right: f64,
    pub margin_top: f64,
    pub margin_bottom: f64,
    pub start_y: f64,
    pub bottom_margin: f64,
    pub font_size: f64,
    pub title_font_size: f64,
    pub section_font_size: f64,
    pub subsection_font_size: f64,
    pub line_width: f64,
    pub line_spacing: f64,
    pub paragraph_spacing: f64,
    pub section_spacing: f64,
    pub subsection_spacing: f64,
}

impl Default for RenderConfig {
    fn default() -> Self {
        RenderConfig {
            page_width: Mm(210.0),  // A4 width
            page_height: Mm(297.0), // A4 height
            margin_left: 10.0,
            margin_right: 10.0,
            margin_top: 10.0,
            margin_bottom: 10.0,
            start_y: 280.0,
            bottom_margin: 20.0,
            font_size: 12.0,
            title_font_size: 20.0,
            section_font_size: 16.0,
            subsection_font_size: 14.0,
            line_width: 190.0,
            line_spacing: 4.0,
            paragraph_spacing: 10.0,
            section_spacing: 15.0,
            subsection_spacing: 10.0,
        }
    }
}
