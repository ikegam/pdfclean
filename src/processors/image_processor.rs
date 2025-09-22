use anyhow::Result;
use markdown::mdast::Node;
use super::traits::{NodeProcessor, ProcessContext};

/// Image preservation processor
/// Ensures that image references ![alt](url) are preserved exactly as-is
/// This is critical for AI-extracted content where images are important
pub struct ImageProcessor {
    name: String,
}

impl ImageProcessor {
    pub fn new() -> Self {
        Self {
            name: "ImageProcessor".to_string(),
        }
    }
}

impl NodeProcessor for ImageProcessor {
    fn should_process(&self, node: &Node) -> bool {
        matches!(node, Node::Image(_))
    }

    fn process_node(&self, node: Node, _context: &ProcessContext) -> Result<Option<Node>> {
        // Images are preserved as-is - no modification needed
        Ok(Some(node))
    }

    fn name(&self) -> &str {
        &self.name
    }
}