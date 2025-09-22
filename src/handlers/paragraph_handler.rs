use anyhow::Result;
use super::traits::{Handler, HandlerContext, MarkdownUnit};
use super::RegexHandler;

pub struct ParagraphHandler {
    inner: RegexHandler,
}

impl ParagraphHandler {
    pub fn new(pattern: &str, replacement: String) -> Result<Self> {
        let inner = RegexHandler::new(
            "ParagraphHandler".to_string(),
            pattern,
            replacement,
        )?.with_unit_filter("paragraph".to_string());

        Ok(Self { inner })
    }
}

impl Handler for ParagraphHandler {
    fn can_handle(&self, unit: &MarkdownUnit) -> bool {
        matches!(unit, MarkdownUnit::Paragraph { .. }) && self.inner.can_handle(unit)
    }

    fn handle(&self, unit: MarkdownUnit, context: &HandlerContext) -> Result<Option<MarkdownUnit>> {
        self.inner.handle(unit, context)
    }

    fn name(&self) -> &str {
        "ParagraphHandler"
    }
}