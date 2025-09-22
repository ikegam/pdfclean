use anyhow::Result;
use super::traits::{Handler, HandlerContext, MarkdownUnit};
use super::RegexHandler;

pub struct HeadingHandler {
    inner: RegexHandler,
}

impl HeadingHandler {
    pub fn new(pattern: &str, replacement: String) -> Result<Self> {
        let inner = RegexHandler::new(
            "HeadingHandler".to_string(),
            pattern,
            replacement,
        )?.with_unit_filter("heading".to_string());

        Ok(Self { inner })
    }
}

impl Handler for HeadingHandler {
    fn can_handle(&self, unit: &MarkdownUnit) -> bool {
        matches!(unit, MarkdownUnit::Heading { .. }) && self.inner.can_handle(unit)
    }

    fn handle(&self, unit: MarkdownUnit, context: &HandlerContext) -> Result<Option<MarkdownUnit>> {
        self.inner.handle(unit, context)
    }

    fn name(&self) -> &str {
        "HeadingHandler"
    }
}