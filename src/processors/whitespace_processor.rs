use anyhow::Result;
use markdown::mdast::Node;
use super::traits::{NodeProcessor, ProcessContext};

/// AI-specialized whitespace processor
/// Cleans excessive whitespace from PDF-extracted markdown content
/// while preserving structured elements like images and tables
pub struct WhitespaceProcessor {
    name: String,
}

impl WhitespaceProcessor {
    pub fn new() -> Self {
        Self {
            name: "WhitespaceProcessor".to_string(),
        }
    }

    /// Clean whitespace from text content
    fn clean_whitespace(&self, text: &str) -> String {
        text.split_whitespace().collect::<Vec<_>>().join("")
    }

}

impl NodeProcessor for WhitespaceProcessor {
    fn should_process(&self, node: &Node) -> bool {
        match node {
            Node::Text(text) => {
                // Skip table-like text and image references
                !(text.value.contains('|') && text.value.matches('|').count() >= 2 && !text.value.starts_with("!["))
            }
            _ => false,
        }
    }

    fn process_node(&self, mut node: Node, _context: &ProcessContext) -> Result<Option<Node>> {
        match &mut node {
            Node::Text(text) => {
                text.value = self.clean_whitespace(&text.value);
                Ok(Some(node))
            }
            _ => Ok(Some(node)),
        }
    }

    fn name(&self) -> &str {
        &self.name
    }
}