use std::sync::Arc;
use markdown_filter::{MarkdownProcessor, WhitespaceHandler, ImageHandler, TableHandler};

/// Helper function to create a processor with all handlers
fn create_processor() -> MarkdownProcessor {
    let mut processor = MarkdownProcessor::new();

    // Add ImageHandler first to protect images from whitespace processing
    let image_handler = ImageHandler::new();
    processor.add_handler(Arc::new(image_handler));

    // Add TableHandler to protect tables from whitespace processing
    let table_handler = TableHandler::new();
    processor.add_handler(Arc::new(table_handler));

    let whitespace_handler = WhitespaceHandler::new();
    processor.add_handler(Arc::new(whitespace_handler));

    processor
}

/// Count the number of image references in markdown content
fn count_images(content: &str) -> usize {
    content.matches("![").count()
}

/// Count the number of table rows in markdown content
fn count_table_rows(content: &str) -> usize {
    content.lines().filter(|line| line.contains('|') && !line.trim().is_empty()).count()
}

#[test]
fn test_basic_whitespace_cleaning() {
    let processor = create_processor();
    let input = include_str!("fixtures/basic_text.md");
    let expected = include_str!("expected/basic_text_cleaned.md");

    let result = processor.process(input).expect("Processing should succeed");

    // Compare the results
    assert_eq!(result.trim(), expected.trim(), "Basic whitespace cleaning failed");
}

#[test]
fn test_image_preservation() {
    let processor = create_processor();
    let input = include_str!("fixtures/with_images.md");
    let expected = include_str!("expected/with_images_cleaned.md");

    let result = processor.process(input).expect("Processing should succeed");

    // Count images to ensure they are preserved
    let input_image_count = count_images(input);
    let result_image_count = count_images(&result);

    assert_eq!(input_image_count, result_image_count,
               "Image count mismatch: input has {}, result has {}",
               input_image_count, result_image_count);

    // Check that specific images are preserved
    assert!(result.contains("![テスト画像1](path/to/image1.jpg)"),
            "Test image 1 not preserved");
    assert!(result.contains("![インライン画像](inline.gif)"),
            "Inline image not preserved");
    assert!(result.contains("![](path/to/no-alt.jpeg)"),
            "No-alt image not preserved");

    // Compare the full results
    assert_eq!(result.trim(), expected.trim(), "Image preservation test failed");
}

#[test]
fn test_table_preservation() {
    let processor = create_processor();
    let input = include_str!("fixtures/with_tables.md");
    let expected = include_str!("expected/with_tables_cleaned.md");

    let result = processor.process(input).expect("Processing should succeed");

    // Count table rows to ensure they are preserved
    let input_table_rows = count_table_rows(input);
    let result_table_rows = count_table_rows(&result);

    assert_eq!(input_table_rows, result_table_rows,
               "Table row count mismatch: input has {}, result has {}",
               input_table_rows, result_table_rows);

    // Check that table structure is preserved (spaces around |)
    assert!(result.contains("| 列1　　　 | 列2　　　　　 | 列3　　　　 |"),
            "Table header not preserved properly");
    assert!(result.contains("| あいうえお | かきくけこ　　　 | さしすせそ |"),
            "Table content not preserved properly");

    // Compare the full results
    assert_eq!(result.trim(), expected.trim(), "Table preservation test failed");
}

#[test]
fn test_complex_document_processing() {
    let processor = create_processor();
    let input = include_str!("fixtures/complex_document.md");
    let expected = include_str!("expected/complex_document_cleaned.md");

    let result = processor.process(input).expect("Processing should succeed");

    // Test image preservation in complex document
    let input_image_count = count_images(input);
    let result_image_count = count_images(&result);

    assert_eq!(input_image_count, result_image_count,
               "Image count mismatch in complex document: input has {}, result has {}",
               input_image_count, result_image_count);

    // Test table preservation in complex document
    let input_table_rows = count_table_rows(input);
    let result_table_rows = count_table_rows(&result);

    assert_eq!(input_table_rows, result_table_rows,
               "Table row count mismatch in complex document: input has {}, result has {}",
               input_table_rows, result_table_rows);

    // Check specific complex elements
    assert!(result.contains("![商品A](product_a.jpg)"),
            "Table-embedded image not preserved");
    assert!(result.contains("| 1  | 商品A　　　 | ![商品A](product_a.jpg) |"),
            "Complex table row not preserved");

    // Compare the full results
    assert_eq!(result.trim(), expected.trim(), "Complex document processing failed");
}

#[test]
fn test_whitespace_only_cleaning() {
    let processor = create_processor();

    // Test with content that has only whitespace changes, no images or tables
    let input = "# テスト　　　見出し\n\n段落　　　テスト　　　です。\n\n最終　　　段落。";
    let expected = "# テスト見出し\n\n段落テストです。\n\n最終段落。";

    let result = processor.process(input).expect("Processing should succeed");

    assert_eq!(result.trim(), expected.trim(), "Whitespace-only cleaning failed");
}

#[test]
fn test_edge_cases() {
    let processor = create_processor();

    // Test empty content
    let result = processor.process("").expect("Processing empty content should succeed");
    assert_eq!(result.trim(), "", "Empty content should remain empty");

    // Test only images
    let image_only = "![test](test.jpg)\n\n![test2](test2.png)";
    let result = processor.process(image_only).expect("Processing image-only content should succeed");
    assert_eq!(count_images(&result), 2, "Image-only content should preserve both images");

    // Test only tables
    let table_only = "| A | B |\n|---|---|\n| 1 | 2 |";
    let result = processor.process(table_only).expect("Processing table-only content should succeed");
    assert!(result.contains("| A | B |"), "Table-only content should preserve table structure");
}