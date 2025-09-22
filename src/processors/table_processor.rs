use anyhow::Result;
use markdown::mdast::Node;
use super::traits::{NodeProcessor, ProcessContext};

/// Table preservation processor
/// Preserves table formatting from PDF-extracted content
/// Tables often get mangled during PDF-to-markdown conversion, so we preserve them as-is
pub struct TableProcessor {
    name: String,
}

impl TableProcessor {
    pub fn new() -> Self {
        Self {
            name: "TableProcessor".to_string(),
        }
    }

}

impl NodeProcessor for TableProcessor {
    fn should_process(&self, node: &Node) -> bool {
        match node {
            Node::Table(_) => true,
            Node::Text(text) => {
                // Protect text that looks like table content
                text.value.contains('|') &&
                text.value.matches('|').count() >= 2 &&
                !text.value.starts_with("![")
            }
            _ => false,
        }
    }

    fn process_node(&self, node: Node, _context: &ProcessContext) -> Result<Option<Node>> {
        // Tables are preserved as-is - no modification needed
        // This includes both proper table nodes and paragraph-based tables
        Ok(Some(node))
    }

    fn name(&self) -> &str {
        &self.name
    }
}