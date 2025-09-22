use regex::Regex;
use anyhow::Result;
use super::traits::{Handler, HandlerContext, MarkdownUnit};

pub struct RegexHandler {
    name: String,
    pattern: Regex,
    replacement: String,
    unit_filter: Option<String>,
}

impl RegexHandler {
    pub fn new(name: String, pattern: &str, replacement: String) -> Result<Self> {
        Ok(Self {
            name,
            pattern: Regex::new(pattern)?,
            replacement,
            unit_filter: None,
        })
    }

    pub fn with_unit_filter(mut self, unit_type: String) -> Self {
        self.unit_filter = Some(unit_type);
        self
    }
}

impl Handler for RegexHandler {
    fn can_handle(&self, unit: &MarkdownUnit) -> bool {
        if let Some(ref filter) = self.unit_filter {
            unit.unit_type() == filter
        } else {
            true
        }
    }

    fn handle(&self, mut unit: MarkdownUnit, _context: &HandlerContext) -> Result<Option<MarkdownUnit>> {
        let content = unit.content();
        let new_content = self.pattern.replace_all(content, &self.replacement).to_string();

        if new_content != content {
            unit.set_content(new_content);
        }

        Ok(Some(unit))
    }

    fn name(&self) -> &str {
        &self.name
    }
}