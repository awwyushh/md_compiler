use std::env;
mod parse_markdown;
use std::io::{BufReader, BufRead, Write};
use std::path::Path;
use std::fs::File;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        2 => parse_markdown::parse_markdown_file(&args[1]),
        _ => {
            println!("Usage: {} <markdown file>", args[0]);
            std::process::exit(1);
        },
    }
}
