use std::sync::Arc;
use std::env;
use std::fs;
use anyhow::Result;
use markdown_filter::{MarkdownProcessor, WhitespaceHandler, ImageHandler, TableHandler};

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: {} <input_file> <output_file>", args[0]);
        std::process::exit(1);
    }

    let input_file = &args[1];
    let output_file = &args[2];

    let content = fs::read_to_string(input_file)?;

    let mut processor = MarkdownProcessor::new();

    // Add ImageHandler first to protect images from whitespace processing
    let image_handler = ImageHandler::new();
    processor.add_handler(Arc::new(image_handler));

    // Add TableHandler to protect tables from whitespace processing
    let table_handler = TableHandler::new();
    processor.add_handler(Arc::new(table_handler));

    let whitespace_handler = WhitespaceHandler::new();
    processor.add_handler(Arc::new(whitespace_handler));

    let cleaned_content = processor.process(&content)?;

    fs::write(output_file, cleaned_content)?;

    println!("Cleaned {} -> {}", input_file, output_file);

    Ok(())
}