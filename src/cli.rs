use clap::{ArgGroup, Parser};

/// RustaTex: A lightweight LaTeX alternative written in Rust
#[derive(Parser, Debug)]
#[command(name = "RustaTex")]
#[command(author = "Your Name <you@example.com>")]
#[command(version = "0.1.0")]
#[command(about = "A lightweight LaTeX alternative written in Rust", long_about = None)]
#[command(group(
    ArgGroup::new("output")
        .required(false)
        .args(&["pdf", "html"]),
))]
pub struct Cli {
    /// Input RustaTex file
    #[arg(short, long, value_name = "FILE", required = true)]
    pub input: String,

    /// Output PDF file
    #[arg(short = 'p', long, value_name = "PDF_FILE", conflicts_with = "html")]
    pub pdf: Option<String>,

    /// Output HTML file
    #[arg(short = 'h', long, value_name = "HTML_FILE", conflicts_with = "pdf")]
    pub html: Option<String>,

    /// Specify the paper size (e.g., A4, Letter)
    #[arg(short, long, value_name = "SIZE", default_value = "A4")]
    pub paper_size: String,

    /// Enable verbose logging
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    pub verbose: bool,

    /// Set the font family (e.g., Times, Arial)
    #[arg(long, value_name = "FONT", default_value = "Times")]
    pub font: String,

    /// Set the font size (in points)
    #[arg(long, value_name = "SIZE", default_value = "12")]
    pub font_size: u32,

    /// Enable debug mode
    #[arg(short = 'd', long, action = clap::ArgAction::SetTrue)]
    pub debug: bool,

    /// Specify the output directory
    #[arg(short, long, value_name = "DIR", default_value = "./output")]
    pub output_dir: String,

    /// Include table of contents
    #[arg(short = 'c', long, action = clap::ArgAction::SetTrue)]
    pub toc: bool,

    /// Define custom macros
    #[arg(short, long, value_name = "MACRO", number_of_values = 1)]
    pub macro_def: Vec<String>,

    /// Set the line spacing
    #[arg(long, value_name = "SPACING", default_value = "1.5")]
    pub line_spacing: f32,

    /// Enable hyperlink support
    #[arg(long, action = clap::ArgAction::SetTrue)]
    pub hyperlinks: bool,

    /// Include bibliography
    #[arg(short = 'b', long, value_name = "BIB_FILE")]
    pub bibliography: Option<String>,

    /// Set the citation style (e.g., APA, MLA)
    #[arg(long, value_name = "STYLE", default_value = "APA")]
    pub citation_style: String,

    /// Enable table numbering
    #[arg(long, action = clap::ArgAction::SetTrue)]
    pub table_numbers: bool,

    /// Enable figure numbering
    #[arg(long, action = clap::ArgAction::SetTrue)]
    pub figure_numbers: bool,

    /// Set margin sizes (e.g., margins=1in)
    #[arg(long, value_name = "MARGINS", default_value = "1in")]
    pub margins: String,

    /// Specify header content
    #[arg(long, value_name = "HEADER")]
    pub header: Option<String>,

    /// Specify footer content
    #[arg(long, value_name = "FOOTER")]
    pub footer: Option<String>,

    /// Enable syntax highlighting for code blocks
    #[arg(long, action = clap::ArgAction::SetTrue)]
    pub syntax_highlighting: bool,

    /// Choose the color theme (e.g., light, dark)
    #[arg(long, value_name = "THEME", default_value = "light")]
    pub theme: String,

    /// Set the page numbering style (e.g., arabic, roman)
    #[arg(long, value_name = "STYLE", default_value = "arabic")]
    pub page_numbering: String,

    /// Enable watermark
    #[arg(long, value_name = "TEXT")]
    pub watermark: Option<String>,

    /// Specify the DPI for images
    #[arg(long, value_name = "DPI", default_value = "300")]
    pub image_dpi: u32,

    /// Enable draft mode (placeholders instead of actual content)
    #[arg(long, action = clap::ArgAction::SetTrue)]
    pub draft: bool,

    /// Set the maximum number of columns
    #[arg(long, value_name = "COLUMNS", default_value = "1")]
    pub columns: u32,

    /// Enable automatic hyphenation
    #[arg(long, action = clap::ArgAction::SetTrue)]
    pub hyphenation: bool,

    /// Specify custom styles file
    #[arg(long, value_name = "STYLES_FILE")]
    pub styles_file: Option<String>,

    /// Enable bookmarks in PDF
    #[arg(long, action = clap::ArgAction::SetTrue)]
    pub bookmarks: bool,

    /// Set the default language for the document
    #[arg(long, value_name = "LANGUAGE", default_value = "en")]
    pub language: String,

    /// Enable compression for output files
    #[arg(long, action = clap::ArgAction::SetTrue)]
    pub compression: bool,
}
