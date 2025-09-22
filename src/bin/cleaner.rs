use std::sync::Arc;
use std::env;
use std::fs;
use anyhow::Result;
use pdfclean::{MarkdownCleaner, WhitespaceProcessor, ImageProcessor, TableProcessor};

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: {} <input_file> <output_file>", args[0]);
        eprintln!("");
        eprintln!("pdfclean - AI-specialized markdown cleaner for PDF-extracted content");
        eprintln!("Removes excessive whitespace while preserving images and tables");
        std::process::exit(1);
    }

    let input_file = &args[1];
    let output_file = &args[2];

    let content = fs::read_to_string(input_file)?;

    let mut cleaner = MarkdownCleaner::new();

    // Add processors in priority order
    // Images and tables are preserved, whitespace is cleaned
    cleaner.add_processor(Arc::new(ImageProcessor::new()));
    cleaner.add_processor(Arc::new(TableProcessor::new()));
    cleaner.add_processor(Arc::new(WhitespaceProcessor::new()));

    let cleaned_content = cleaner.clean(&content)?;

    fs::write(output_file, cleaned_content)?;

    println!("Cleaned {} -> {}", input_file, output_file);

    Ok(())
}