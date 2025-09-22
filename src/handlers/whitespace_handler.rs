use anyhow::Result;
use super::traits::{Handler, HandlerContext, MarkdownUnit};

pub struct WhitespaceHandler {
    name: String,
}

impl WhitespaceHandler {
    pub fn new() -> Self {
        Self {
            name: "WhitespaceHandler".to_string(),
        }
    }

    fn clean_whitespace(&self, content: &str) -> String {
        content.split_whitespace().collect::<Vec<_>>().join("")
    }
}

impl Handler for WhitespaceHandler {
    fn can_handle(&self, unit: &MarkdownUnit) -> bool {
        // Skip image units - they should be handled by ImageHandler
        !matches!(unit, MarkdownUnit::Image { .. })
    }

    fn handle(&self, mut unit: MarkdownUnit, _context: &HandlerContext) -> Result<Option<MarkdownUnit>> {
        match &mut unit {
            MarkdownUnit::Heading { content, .. } => {
                *content = self.clean_whitespace(content);
            }
            MarkdownUnit::Paragraph { content } => {
                *content = self.clean_whitespace(content);
            }
            MarkdownUnit::CodeBlock { content, .. } => {
                *content = self.clean_whitespace(content);
            }
            MarkdownUnit::BlockQuote { content } => {
                *content = self.clean_whitespace(content);
            }
            MarkdownUnit::Raw { content } => {
                *content = self.clean_whitespace(content);
            }
            MarkdownUnit::List { items } => {
                for item in items {
                    *item = self.clean_whitespace(item);
                }
            }
            MarkdownUnit::Image { .. } => {
                // Images should not be processed by this handler
                // This case should not occur due to can_handle() check
            }
        }

        Ok(Some(unit))
    }

    fn name(&self) -> &str {
        &self.name
    }
}