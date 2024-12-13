use std::path::Path;
use std::fs::File;
use std::io::{BufReader, BufRead, Write};

pub fn parse_markdown_file(file_path: &str) {
    println!("Parsing markdown file: {}", file_path);

    let input_filename = Path::new(file_path);
    
    let file = File::open(&input_filename).expect("File not found");

    let mut htag: bool = false;
    let mut ptag: bool = false;

    let mut tokens: Vec<String> = Vec::new();

    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line_contents = line.unwrap();
        let mut output_line = String::new();

        // Check for blockquote first
        let is_blockquote = line_contents.starts_with('>');
        let content = if is_blockquote {
            line_contents[1..].trim().to_string()
        } else {
            line_contents
        };

        let heading_level = content.chars()
            .take_while(|&c| c == '#')
            .count();

        // Process the line content for styles and links
        let processed_content = process_text_styles(&content);

        match heading_level {
            1..=3 => {
                if ptag {
                    ptag = false;
                    output_line.push_str("</p>\n");
                } 
                if htag {
                    htag = false;
                    output_line.push_str("</h1>\n");
                }
        
                htag = true;
                output_line.push_str(&format!("<h{}>", heading_level));
                output_line.push_str(processed_content[heading_level + 1..].trim());
                output_line.push_str(&format!("</h{}>\n", heading_level));
                htag = false;
            },
        
            0 | _ => {
                if !ptag {
                    ptag = true;
                    if is_blockquote {
                        output_line.push_str("<blockquote><p>");
                    } else {
                        output_line.push_str("<p>");
                    }
                } 
                output_line.push_str(&processed_content);
            }
        };

        if ptag {
            ptag = false;
            if is_blockquote {
                output_line.push_str("</p></blockquote>\n");
            } else {
                output_line.push_str("</p>\n");
            }
        }

        if output_line != "<p></p>\n" {
            tokens.push(output_line);
        }
    }

    let mut output_file = File::create("output.html").expect("Failed to create output file");
    for token in tokens {
        output_file.write_all(token.as_bytes()).expect("Failed to write to output file");
    }

    println!("[+] Markdown file parsed successfully and saved to output.html");
}

fn process_text_styles(text: &str) -> String {
    let mut result = String::new();
    let mut chars = text.chars().peekable();
    let mut in_bold = false;
    let mut in_italic = false;
    let mut in_strike = false;
    let mut link_text = String::new();
    let mut collecting_link_text = false;
    let mut link_url = String::new();
    let mut collecting_link_url = false;

    while let Some(current) = chars.next() {
        match current {
            '[' => {
                collecting_link_text = true;
                continue;
            },
            ']' => {
                if chars.peek() == Some(&'(') {
                    collecting_link_text = false;
                    chars.next(); // consume '('
                    collecting_link_url = true;
                    continue;
                } else {
                    result.push('[');
                    result.push_str(&link_text);
                    result.push(']');
                    link_text.clear();
                }
            },
            ')' if collecting_link_url => {
                collecting_link_url = false;
                result.push_str(&format!("<a href=\"{}\">{}</a>", link_url, link_text));
                link_text.clear();
                link_url.clear();
                continue;
            },
            '~' => {
                if chars.peek() == Some(&'~') {
                    chars.next(); // consume second ~
                    if !in_strike {
                        result.push_str("<del>");
                        in_strike = true;
                    } else {
                        result.push_str("</del>");
                        in_strike = false;
                    }
                    continue;
                }
            },
            '*' | '_' => {
                if collecting_link_text {
                    link_text.push(current);
                    continue;
                }
                if collecting_link_url {
                    link_url.push(current);
                    continue;
                }
                if !in_bold {
                    result.push_str("<strong>");
                    in_bold = true;
                } else {
                    result.push_str("</strong>");
                    in_bold = false;
                }
                if !in_italic {
                    result.push_str("<em>");
                    in_italic = true;
                } else {
                    result.push_str("</em>");
                    in_italic = false;
                }
            },
            _ => {
                if collecting_link_text {
                    link_text.push(current);
                } else if collecting_link_url {
                    link_url.push(current);
                } else {
                    result.push(current);
                }
            }
        }
    }

    // Handle unclosed tags
    if in_bold {
        result.push_str("</strong>");
    }
    if in_italic {
        result.push_str("</em>");
    }
    if in_strike {
        result.push_str("</del>");
    }

    result
}

