mod cli;
mod document;
mod errors;
mod parser;
mod renderer;
mod styles;
use clap::Parser;
use cli::Cli;
use env_logger;

use log::{debug, error, info};
use parser::parse_input;
use printpdf::Mm;
use regex::Regex;
use renderer::{render_pdf, RenderConfig};

use std::{path::Path, process};

fn main() {
    // Initialize the logger
    env_logger::init();

    let cli = Cli::parse();

    if cli.verbose {
        info!("Starting RustaTex with the following configuration:");
        info!("{:#?}", cli);
    }

    // Read input file
    let input_content = match std::fs::read_to_string(&cli.input) {
        Ok(content) => content,
        Err(err) => {
            error!("Error reading input file {}: {}", &cli.input, err);
            eprintln!("Error reading input file {}: {}", &cli.input, err);
            process::exit(1);
        }
    };

    // Parse input
    let mut document = match parse_input(&input_content) {
        Ok(doc) => doc,
        Err(err) => {
            error!("Error parsing input file: {}", err);
            eprintln!("Error parsing input file: {}", err);
            process::exit(1);
        }
    };

    // Handle macros from CLI
    for macro_def in &cli.macro_def {
        let parts: Vec<&str> = macro_def.split('=').collect();
        if parts.len() == 2 {
            let name = parts[0].trim().to_string();
            let value = parts[1].trim().to_string();
            document.define_macro(name.clone(), value.clone());
            debug!("Defined macro from CLI: {} = {}", name, value);
        } else {
            error!("Invalid macro definition: {}", macro_def);
            eprintln!("Invalid macro definition: {}", macro_def);
            process::exit(1);
        }
    }

    // Set up rendering configuration based on CLI
    let render_config = RenderConfig {
        page_width: match cli.paper_size.to_uppercase().as_str() {
            "A4" => Mm(210.0),
            "LETTER" => Mm(215.9),
            _ => Mm(210.0), // Default to A4
        },
        page_height: match cli.paper_size.to_uppercase().as_str() {
            "A4" => Mm(297.0),
            "LETTER" => Mm(279.4),
            _ => Mm(297.0), // Default to A4
        },
        margin_left: parse_margin(&cli.margins).unwrap_or(25.4), // 1in = 25.4mm
        margin_right: parse_margin(&cli.margins).unwrap_or(25.4),
        margin_top: parse_margin(&cli.margins).unwrap_or(25.4),
        margin_bottom: parse_margin(&cli.margins).unwrap_or(25.4),
        start_y: 280.0,      // Starting Y position for rendering text
        bottom_margin: 20.0, // Minimum Y position before adding a new page
        font_size: cli.font_size as f64,
        title_font_size: 20.0,
        section_font_size: 16.0,
        subsection_font_size: 14.0,
        line_width: 190.0,
        line_spacing: 4.0,
        paragraph_spacing: 10.0,
        section_spacing: 15.0,
        subsection_spacing: 10.0,
    };

    // Determine output path
    let output_pdf = if let Some(pdf_path) = cli.pdf.clone() {
        pdf_path
    } else {
        // Default output path
        "./output/output.pdf".to_string()
    };

    // Create output directory if it doesn't exist
    if let Some(parent) = Path::new(&output_pdf).parent() {
        if !parent.exists() {
            if let Err(e) = std::fs::create_dir_all(parent) {
                error!(
                    "Failed to create output directory {}: {}",
                    parent.display(),
                    e
                );
                eprintln!(
                    "Failed to create output directory {}: {}",
                    parent.display(),
                    e
                );
                process::exit(1);
            }
        }
    }

    // Render PDF
    if let Err(e) = render_pdf(&document, &output_pdf, &render_config) {
        error!("Error generating PDF: {}", e);
        eprintln!("Error generating PDF: {}", e);
        process::exit(1);
    } else {
        info!("PDF generated successfully at {}", &output_pdf);
        println!("PDF generated successfully at {}", &output_pdf);
    }
}

/// Parses margin string (e.g., "1in") and returns value in mm.
/// Supports "in" for inches and "cm" for centimeters.
fn parse_margin(margin_str: &str) -> Option<f64> {
    lazy_static::lazy_static! {
        static ref RE: Regex = Regex::new(r"(?i)^(\d+(\.\d+)?)(in|cm)$").unwrap();
    }

    if let Some(caps) = RE.captures(margin_str) {
        let value: f64 = caps.get(1)?.as_str().parse().ok()?;
        let unit = caps.get(3)?.as_str().to_lowercase();
        match unit.as_str() {
            "in" => Some(value * 25.4),
            "cm" => Some(value * 10.0),
            _ => None,
        }
    } else {
        // Default to 25.4mm if parsing fails
        Some(25.4)
    }
}
