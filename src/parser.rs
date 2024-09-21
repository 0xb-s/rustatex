use crate::document::*;
use crate::errors::RustaTexError;
use log::{debug, error};
use pest::iterators::Pair;
use pest::Parser as PestParser;
use regex::Regex;

#[derive(pest_derive::Parser)]
#[grammar = "grammar.pest"]
pub struct RustaTexParser;

/// Parses the input RustaTex content into a Document structure.
pub fn parse_input(input: &str) -> Result<Document, RustaTexError> {
    debug!("Starting parsing of the document.");
    let parsed = RustaTexParser::parse(Rule::document, input)?;
    let mut document = Document::new();

    for pair in parsed {
        match pair.as_rule() {
            Rule::document => {
                for inner_pair in pair.into_inner() {
                    match inner_pair.as_rule() {
                        Rule::command => {
                            debug!("Parsing command: {:?}", inner_pair.as_str());
                            parse_command(inner_pair, &mut document)?;
                        }
                        Rule::paragraph => {
                            debug!("Parsing paragraph.");
                            let paragraph = parse_paragraph(inner_pair)?;
                            document.add_element(DocumentElement::Paragraph(paragraph));
                        }
                        Rule::EOI => (),
                        _ => {
                            error!("Unexpected rule in document: {:?}", inner_pair.as_rule());
                            return Err(RustaTexError::InvalidSyntax(format!(
                                "Unexpected rule in document: {:?}",
                                inner_pair.as_rule()
                            )));
                        }
                    }
                }
            }
            _ => {
                error!("Unexpected top-level rule: {:?}", pair.as_rule());
                return Err(RustaTexError::InvalidSyntax(format!(
                    "Unexpected top-level rule: {:?}",
                    pair.as_rule()
                )));
            }
        }
    }

    Ok(document)
}

/// Parses a single command and updates the Document accordingly.
fn parse_command(pair: Pair<Rule>, document: &mut Document) -> Result<(), RustaTexError> {
    let span = pair.as_span();
    let position = span.start_pos();

    let mut inner = pair.into_inner();

    // Skip the backslash
    let _backslash = inner.next();

    // Now get the identifier (command) and argument
    let command = match inner.next() {
        Some(cmd) => cmd.as_str(),
        None => {
            error!(
                "Missing command identifier at line {}, column {}",
                position.line_col().0,
                position.line_col().1
            );
            return Err(RustaTexError::InvalidSyntax(format!(
                "Missing command identifier at line {}, column {}",
                position.line_col().0,
                position.line_col().1
            )));
        }
    };

    let argument = match inner.next() {
        Some(arg) => arg.as_str(),
        None => {
            error!(
                "Missing argument for command '{}' at line {}, column {}",
                command,
                position.line_col().0,
                position.line_col().1
            );
            return Err(RustaTexError::InvalidSyntax(format!(
                "Missing argument for command '{}' at line {}, column {}",
                command,
                position.line_col().0,
                position.line_col().1
            )));
        }
    };

    match command {
        "section" => {
            let section = Section {
                title: substitute_macros(argument, document)?,
                elements: Vec::new(),
                label: None,
            };
            document.add_element(DocumentElement::Section(section));
            debug!("Added section: {}", argument);
        }
        "subsection" => {
            let subsection = Subsection {
                title: substitute_macros(argument, document)?,
                elements: Vec::new(),
                label: None,
            };
            document.add_element(DocumentElement::Subsection(subsection));
            debug!("Added subsection: {}", argument);
        }
        "paragraph" => {
            let paragraph = Paragraph {
                text: substitute_macros(argument, document)?,
                style: None,
            };
            document.add_element(DocumentElement::Paragraph(paragraph));
            debug!("Added paragraph.");
        }
        "macro" => {
            // Define a new macro
            let parts: Vec<&str> = argument.split('=').collect();
            if parts.len() == 2 {
                let name = parts[0].trim().to_string();
                let value = parts[1].trim().to_string();
                document.define_macro(name.clone(), value.clone());
                debug!("Defined macro: {} = {}", name, value);
            } else {
                error!(
                    "Invalid macro definition at line {}, column {}: {}",
                    position.line_col().0,
                    position.line_col().1,
                    argument
                );
                return Err(RustaTexError::InvalidSyntax(format!(
                    "Invalid macro definition at line {}, column {}: {}",
                    position.line_col().0,
                    position.line_col().1,
                    argument
                )));
            }
        }
        "title" => {
            document.set_title(substitute_macros(argument, document)?);
            debug!("Set title: {}", argument);
        }
        "author" => {
            document.set_author(substitute_macros(argument, document)?);
            debug!("Set author: {}", argument);
        }
        "date" => {
            document.set_date(substitute_macros(argument, document)?);
            debug!("Set date: {}", argument);
        }
        // Add more commands as needed
        _ => {
            error!(
                "Unknown command '{}' at line {}, column {}",
                command,
                position.line_col().0,
                position.line_col().1
            );
            return Err(RustaTexError::UnknownCommand(format!(
                "{} at line {}, column {}",
                command,
                position.line_col().0,
                position.line_col().1
            )));
        }
    }

    Ok(())
}

/// Parses a paragraph and returns a Paragraph struct.
fn parse_paragraph(pair: Pair<Rule>) -> Result<Paragraph, RustaTexError> {
    let text = pair.as_str().trim().to_string();
    Ok(Paragraph { text, style: None })
}

/// Substitutes macros within the given text using the Document's macro definitions.
fn substitute_macros(text: &str, document: &Document) -> Result<String, RustaTexError> {
    let re = Regex::new(r"\\(\w+)").unwrap();
    let result = re.replace_all(text, |caps: &regex::Captures| {
        let macro_name = &caps[1];
        match document.get_macro(macro_name) {
            Some(val) => val.clone(),
            None => caps[0].to_string(), 
        }
    });
    Ok(result.to_string())
}
