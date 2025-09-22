use anyhow::Result;
use super::traits::{Handler, HandlerContext, MarkdownUnit};

pub struct TableHandler {
    name: String,
}

impl TableHandler {
    pub fn new() -> Self {
        Self {
            name: "TableHandler".to_string(),
        }
    }
}

impl Handler for TableHandler {
    fn can_handle(&self, unit: &MarkdownUnit) -> bool {
        matches!(unit, MarkdownUnit::Table { .. })
    }

    fn handle(&self, unit: MarkdownUnit, _context: &HandlerContext) -> Result<Option<MarkdownUnit>> {
        // Don't modify table units - just pass them through unchanged
        Ok(Some(unit))
    }

    fn name(&self) -> &str {
        &self.name
    }
}