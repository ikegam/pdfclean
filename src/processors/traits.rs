use anyhow::Result;
use markdown::mdast::Node;

#[derive(Debug, Default)]
pub struct ProcessContext {
    pub file_path: Option<String>,
}

/// AI-specialized markdown cleaner trait
/// Works directly with AST nodes for better performance and simpler logic
pub trait NodeProcessor: Send + Sync {
    /// Check if this processor should handle the given node
    fn should_process(&self, node: &Node) -> bool;

    /// Process the node and return the modified node or None to remove it
    fn process_node(&self, node: Node, context: &ProcessContext) -> Result<Option<Node>>;

    /// Get the name of this processor for debugging
    fn name(&self) -> &str;
}