use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub enum DocumentElement {
    Section(Section),
    Subsection(Subsection),
    Paragraph(Paragraph),
    // Future elements: List, Table, Figure, Equation, CodeBlock, etc.
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Section {
    pub title: String,
    pub elements: Vec<DocumentElement>,
    pub label: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Subsection {
    pub title: String,
    pub elements: Vec<DocumentElement>,
    pub label: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Paragraph {
    pub text: String,
    pub style: Option<Style>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Style {
    pub font: Option<String>,
    pub font_size: Option<u32>,
    pub color: Option<String>,
    pub bold: bool,
    pub italic: bool,
    // Add more style attributes as needed
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Document {
    pub title: Option<String>,
    pub author: Option<String>,
    pub date: Option<String>,
    pub elements: Vec<DocumentElement>,
    pub macros: HashMap<String, String>,
}

impl Document {
    pub fn new() -> Self {
        Document {
            title: None,
            author: None,
            date: None,
            elements: Vec::new(),
            macros: HashMap::new(),
        }
    }

    pub fn add_element(&mut self, element: DocumentElement) {
        self.elements.push(element);
    }

    pub fn set_title(&mut self, title: String) {
        self.title = Some(title);
    }

    pub fn set_author(&mut self, author: String) {
        self.author = Some(author);
    }

    pub fn set_date(&mut self, date: String) {
        self.date = Some(date);
    }

    pub fn define_macro(&mut self, name: String, value: String) {
        self.macros.insert(name, value);
    }

    pub fn get_macro(&self, name: &str) -> Option<&String> {
        self.macros.get(name)
    }
}
