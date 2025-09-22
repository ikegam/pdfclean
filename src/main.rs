use std::sync::Arc;
use anyhow::Result;
use markdown_filter::{MarkdownProcessor, RegexHandler, HeadingHandler, ParagraphHandler};

fn main() -> Result<()> {
    let sample_markdown = r#"# Hello World

This is a paragraph with some **bold** text.

## Second Heading

Another paragraph here with *italic* text.

```rust
fn main() {
    println!("Hello");
}
```

> This is a blockquote
> with multiple lines

- Item 1
- Item 2
- Item 3
"#;

    let mut processor = MarkdownProcessor::new();

    let heading_filter = HeadingHandler::new(r"Hello", "Hi".to_string())?;
    processor.add_handler(Arc::new(heading_filter));

    let paragraph_filter = ParagraphHandler::new(r"\*\*(.+?)\*\*", "**$1 (FILTERED)**".to_string())?;
    processor.add_handler(Arc::new(paragraph_filter));

    let general_filter = RegexHandler::new(
        "italic_filter".to_string(),
        r"\*(.+?)\*",
        "_$1_".to_string()
    )?;
    processor.add_handler(Arc::new(general_filter));

    println!("=== Original ===");
    println!("{}", sample_markdown);

    println!("\n=== Processed ===");
    let result = processor.process(sample_markdown)?;
    println!("{}", result);

    Ok(())
}
