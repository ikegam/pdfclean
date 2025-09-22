use std::env;
use std::fs;
use std::io::{self, Read};
use std::sync::Arc;
use anyhow::Result;
use pdfclean::{MarkdownCleaner, WhitespaceProcessor, ImageProcessor, TableProcessor};

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    let input_content = match args.len() {
        1 => {
            // Read from stdin
            let mut buffer = String::new();
            io::stdin().read_to_string(&mut buffer)?;
            buffer
        }
        2 => {
            // Read from file, output to stdout
            fs::read_to_string(&args[1])?
        }
        3 => {
            // Read from file, output to file
            fs::read_to_string(&args[1])?
        }
        _ => {
            eprintln!("Usage:");
            eprintln!("  {} [input_file] [output_file]", args[0]);
            eprintln!("  {} [input_file]                (output to stdout)", args[0]);
            eprintln!("  {}                             (stdin to stdout)", args[0]);
            std::process::exit(1);
        }
    };

    // Create cleaner with all processors
    let mut cleaner = MarkdownCleaner::new();
    cleaner.add_processor(Arc::new(WhitespaceProcessor::new()));
    cleaner.add_processor(Arc::new(ImageProcessor::new()));
    cleaner.add_processor(Arc::new(TableProcessor::new()));

    // Process the content
    let cleaned_content = cleaner.process(&input_content)?;

    // Output the result
    match args.len() {
        3 => {
            // Write to output file
            fs::write(&args[2], cleaned_content)?;
            eprintln!("Cleaned content written to {}", args[2]);
        }
        _ => {
            // Output to stdout
            print!("{}", cleaned_content);
        }
    }

    Ok(())
}