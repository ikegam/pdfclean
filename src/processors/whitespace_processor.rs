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
    /// Only processes full-width character regions, preserves spaces in half-width alphabet regions
    fn clean_whitespace(&self, text: &str) -> String {
        let mut result = String::new();
        let mut chars = text.chars().peekable();

        while let Some(ch) = chars.next() {
            if ch.is_whitespace() {
                // Check if we're in a full-width character context
                let prev_is_fullwidth = result.chars().last()
                    .map(|c| self.is_fullwidth_char(c))
                    .unwrap_or(false);
                let next_is_fullwidth = chars.peek()
                    .map(|c| self.is_fullwidth_char(*c))
                    .unwrap_or(false);

                // Only remove whitespace if both adjacent characters are full-width
                if prev_is_fullwidth && next_is_fullwidth {
                    // Skip excessive whitespace between full-width characters
                    continue;
                } else {
                    // Preserve single space for half-width regions
                    if !result.ends_with(' ') {
                        result.push(' ');
                    }
                }
            } else {
                result.push(ch);
            }
        }

        result.trim().to_string()
    }

    /// Check if a character is full-width (CJK characters, full-width punctuation, etc.)
    fn is_fullwidth_char(&self, ch: char) -> bool {
        match ch as u32 {
            // CJK Unified Ideographs
            0x4E00..=0x9FFF |
            // Hiragana
            0x3040..=0x309F |
            // Katakana
            0x30A0..=0x30FF |
            // Full-width forms
            0xFF00..=0xFFEF |
            // CJK Symbols and Punctuation
            0x3000..=0x303F |
            // Also consider digits as "cleanable" when next to CJK
            0x0030..=0x0039 => true,
            _ => false,
        }
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