use std::sync::Arc;
use anyhow::Result;
use markdown::{mdast::Node, ParseOptions, to_mdast};
use crate::processors::{NodeProcessor, ProcessContext};

/// AI-specialized PDF markdown cleaner
/// Designed to clean up markdown content extracted from PDFs by AI tools
/// while preserving critical structures like images and tables
pub struct MarkdownCleaner {
    processors: Vec<Arc<dyn NodeProcessor>>,
}

impl MarkdownCleaner {
    pub fn new() -> Self {
        Self {
            processors: Vec::new(),
        }
    }

    pub fn add_processor(&mut self, processor: Arc<dyn NodeProcessor>) {
        self.processors.push(processor);
    }

    pub fn clean(&self, markdown_text: &str) -> Result<String> {
        let parse_options = ParseOptions::default();
        let mut root = to_mdast(markdown_text, &parse_options)
            .map_err(|e| anyhow::anyhow!("Failed to parse markdown: {}", e))?;

        let context = ProcessContext::default();
        self.process_node_recursively(&mut root, &context)?;

        // Convert back to markdown using the same library's serializer
        // Since to_markdown is not available, we need to implement our own serialization
        Ok(self.node_to_markdown(&root))
    }

    fn process_node_recursively(&self, node: &mut Node, context: &ProcessContext) -> Result<()> {
        // Process current node with all applicable processors
        for processor in &self.processors {
            if processor.should_process(node) {
                if let Some(processed) = processor.process_node(node.clone(), context)? {
                    *node = processed;
                }
            }
        }

        // Recursively process children, but skip code blocks to preserve formatting
        match node {
            Node::Root(root) => {
                for child in &mut root.children {
                    self.process_node_recursively(child, context)?;
                }
            }
            Node::Paragraph(para) => {
                for child in &mut para.children {
                    self.process_node_recursively(child, context)?;
                }
            }
            Node::Heading(heading) => {
                for child in &mut heading.children {
                    self.process_node_recursively(child, context)?;
                }
            }
            Node::Blockquote(quote) => {
                for child in &mut quote.children {
                    self.process_node_recursively(child, context)?;
                }
            }
            Node::List(list) => {
                for child in &mut list.children {
                    self.process_node_recursively(child, context)?;
                }
            }
            Node::ListItem(item) => {
                for child in &mut item.children {
                    self.process_node_recursively(child, context)?;
                }
            }
            Node::Table(table) => {
                for child in &mut table.children {
                    self.process_node_recursively(child, context)?;
                }
            }
            Node::TableRow(row) => {
                for child in &mut row.children {
                    self.process_node_recursively(child, context)?;
                }
            }
            Node::TableCell(cell) => {
                for child in &mut cell.children {
                    self.process_node_recursively(child, context)?;
                }
            }
            // Skip processing children of code blocks to preserve formatting
            Node::Code(_) => {
                // Do not process children of code blocks
            }
            // Terminal nodes - no children to process
            Node::Text(_) | Node::Image(_) => {}
            _ => {} // Handle other node types if needed
        }

        Ok(())
    }

    fn node_to_markdown(&self, node: &Node) -> String {
        match node {
            Node::Root(root) => {
                root.children.iter()
                    .map(|child| self.node_to_markdown(child))
                    .collect::<Vec<_>>()
                    .join("\n\n")
            }
            Node::Text(text) => text.value.clone(),
            Node::Heading(heading) => {
                let content = heading.children.iter()
                    .map(|child| self.node_to_markdown(child))
                    .collect::<Vec<_>>()
                    .join("");
                format!("{} {}", "#".repeat(heading.depth as usize), content)
            }
            Node::Paragraph(paragraph) => {
                paragraph.children.iter()
                    .map(|child| self.node_to_markdown(child))
                    .collect::<Vec<_>>()
                    .join("")
            }
            Node::Image(image) => {
                format!("![{}]({})", image.alt, image.url)
            }
            Node::Code(code) => {
                if let Some(lang) = &code.lang {
                    format!("```{}\n{}\n```", lang, code.value)
                } else {
                    format!("```\n{}\n```", code.value)
                }
            }
            Node::Blockquote(quote) => {
                quote.children.iter()
                    .map(|child| self.node_to_markdown(child))
                    .collect::<Vec<_>>()
                    .join("")
                    .lines()
                    .map(|line| format!("> {}", line))
                    .collect::<Vec<_>>()
                    .join("\n")
            }
            Node::List(list) => {
                list.children.iter()
                    .map(|child| self.node_to_markdown(child))
                    .collect::<Vec<_>>()
                    .join("\n")
            }
            Node::ListItem(item) => {
                let content = item.children.iter()
                    .map(|child| self.node_to_markdown(child))
                    .collect::<Vec<_>>()
                    .join("");
                format!("- {}", content)
            }
            Node::Table(table) => {
                table.children.iter()
                    .map(|child| self.node_to_markdown(child))
                    .collect::<Vec<_>>()
                    .join("\n")
            }
            Node::TableRow(row) => {
                let cells = row.children.iter()
                    .map(|child| self.node_to_markdown(child))
                    .collect::<Vec<_>>();
                format!("| {} |", cells.join(" | "))
            }
            Node::TableCell(cell) => {
                cell.children.iter()
                    .map(|child| self.node_to_markdown(child))
                    .collect::<Vec<_>>()
                    .join("")
            }
            _ => String::new(),
        }
    }
}