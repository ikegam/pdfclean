use std::sync::Arc;
use anyhow::Result;
use markdown::{mdast::Node, ParseOptions, to_mdast};
use crate::handlers::{Handler, HandlerContext, MarkdownUnit};

pub struct MarkdownProcessor {
    handlers: Vec<Arc<dyn Handler>>,
}

impl MarkdownProcessor {
    pub fn new() -> Self {
        Self {
            handlers: Vec::new(),
        }
    }

    pub fn add_handler(&mut self, handler: Arc<dyn Handler>) {
        self.handlers.push(handler);
    }

    pub fn process(&self, markdown_text: &str) -> Result<String> {
        let parse_options = ParseOptions::default();
        let root = to_mdast(markdown_text, &parse_options)
            .map_err(|e| anyhow::anyhow!("Failed to parse markdown: {}", e))?;

        let mut units = self.extract_units(&root)?;
        let context = HandlerContext::default();

        for unit in &mut units {
            for handler in &self.handlers {
                if handler.can_handle(unit) {
                    if let Some(processed_unit) = handler.handle(unit.clone(), &context)? {
                        *unit = processed_unit;
                    }
                }
            }
        }

        Ok(self.units_to_markdown(units))
    }

    fn extract_units(&self, root: &Node) -> Result<Vec<MarkdownUnit>> {
        let mut units = Vec::new();

        if let Node::Root(root) = root {
            for child in &root.children {
                if let Some(unit) = self.node_to_unit(child) {
                    units.push(unit);
                }
            }
        }

        Ok(units)
    }

    fn node_to_unit(&self, node: &Node) -> Option<MarkdownUnit> {
        match node {
            Node::Heading(heading) => {
                let content = self.extract_text_content(node);
                Some(MarkdownUnit::Heading {
                    level: heading.depth as u8,
                    content,
                })
            }
            Node::Paragraph(_) => {
                let content = self.extract_text_content(node);
                Some(MarkdownUnit::Paragraph { content })
            }
            Node::Code(code) => {
                Some(MarkdownUnit::CodeBlock {
                    lang: code.lang.clone(),
                    content: code.value.clone(),
                })
            }
            Node::Blockquote(_) => {
                let content = self.extract_text_content(node);
                Some(MarkdownUnit::BlockQuote { content })
            }
            Node::List(_) => {
                let items = self.extract_list_items(node);
                Some(MarkdownUnit::List { items })
            }
            _ => {
                let content = self.extract_text_content(node);
                if !content.trim().is_empty() {
                    Some(MarkdownUnit::Raw { content })
                } else {
                    None
                }
            }
        }
    }

    fn extract_text_content(&self, node: &Node) -> String {
        match node {
            Node::Text(text) => text.value.clone(),
            Node::Heading(heading) => {
                heading.children.iter()
                    .map(|child| self.extract_text_content(child))
                    .collect::<Vec<_>>()
                    .join("")
            }
            Node::Paragraph(paragraph) => {
                paragraph.children.iter()
                    .map(|child| self.extract_text_content(child))
                    .collect::<Vec<_>>()
                    .join("")
            }
            Node::Blockquote(quote) => {
                quote.children.iter()
                    .map(|child| self.extract_text_content(child))
                    .collect::<Vec<_>>()
                    .join("\n")
            }
            Node::Strong(strong) => {
                strong.children.iter()
                    .map(|child| self.extract_text_content(child))
                    .collect::<Vec<_>>()
                    .join("")
            }
            Node::Emphasis(emphasis) => {
                emphasis.children.iter()
                    .map(|child| self.extract_text_content(child))
                    .collect::<Vec<_>>()
                    .join("")
            }
            _ => String::new(),
        }
    }

    fn extract_list_items(&self, node: &Node) -> Vec<String> {
        match node {
            Node::List(list) => {
                list.children.iter()
                    .filter_map(|child| {
                        if let Node::ListItem(item) = child {
                            let content = item.children.iter()
                                .map(|child| self.extract_text_content(child))
                                .collect::<Vec<_>>()
                                .join(" ");
                            Some(content)
                        } else {
                            None
                        }
                    })
                    .collect()
            }
            _ => Vec::new(),
        }
    }

    fn units_to_markdown(&self, units: Vec<MarkdownUnit>) -> String {
        units.into_iter()
            .map(|unit| self.unit_to_markdown_string(unit))
            .collect::<Vec<_>>()
            .join("\n\n")
    }

    fn unit_to_markdown_string(&self, unit: MarkdownUnit) -> String {
        match unit {
            MarkdownUnit::Heading { level, content } => {
                format!("{} {}", "#".repeat(level as usize), content)
            }
            MarkdownUnit::Paragraph { content } => content,
            MarkdownUnit::CodeBlock { lang, content } => {
                if let Some(lang) = lang {
                    format!("```{}\n{}\n```", lang, content)
                } else {
                    format!("```\n{}\n```", content)
                }
            }
            MarkdownUnit::List { items } => {
                items.into_iter()
                    .map(|item| format!("- {}", item))
                    .collect::<Vec<_>>()
                    .join("\n")
            }
            MarkdownUnit::BlockQuote { content } => {
                content.lines()
                    .map(|line| format!("> {}", line))
                    .collect::<Vec<_>>()
                    .join("\n")
            }
            MarkdownUnit::Raw { content } => content,
        }
    }
}