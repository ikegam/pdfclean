use anyhow::Result;
use super::traits::{Handler, HandlerContext, MarkdownUnit};

pub struct ImageHandler {
    name: String,
}

impl ImageHandler {
    pub fn new() -> Self {
        Self {
            name: "ImageHandler".to_string(),
        }
    }
}

impl Handler for ImageHandler {
    fn can_handle(&self, unit: &MarkdownUnit) -> bool {
        matches!(unit, MarkdownUnit::Image { .. })
    }

    fn handle(&self, unit: MarkdownUnit, _context: &HandlerContext) -> Result<Option<MarkdownUnit>> {
        // Don't modify image units - just pass them through unchanged
        Ok(Some(unit))
    }

    fn name(&self) -> &str {
        &self.name
    }
}