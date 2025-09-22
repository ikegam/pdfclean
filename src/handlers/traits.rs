use anyhow::Result;

#[derive(Debug, Clone)]
pub enum MarkdownUnit {
    Heading { level: u8, content: String },
    Paragraph { content: String },
    CodeBlock { lang: Option<String>, content: String },
    List { items: Vec<String> },
    BlockQuote { content: String },
    Raw { content: String },
}

impl MarkdownUnit {
    pub fn content(&self) -> &str {
        match self {
            MarkdownUnit::Heading { content, .. } => content,
            MarkdownUnit::Paragraph { content } => content,
            MarkdownUnit::CodeBlock { content, .. } => content,
            MarkdownUnit::List { .. } => "",
            MarkdownUnit::BlockQuote { content } => content,
            MarkdownUnit::Raw { content } => content,
        }
    }

    pub fn set_content(&mut self, new_content: String) {
        match self {
            MarkdownUnit::Heading { content, .. } => *content = new_content,
            MarkdownUnit::Paragraph { content } => *content = new_content,
            MarkdownUnit::CodeBlock { content, .. } => *content = new_content,
            MarkdownUnit::BlockQuote { content } => *content = new_content,
            MarkdownUnit::Raw { content } => *content = new_content,
            MarkdownUnit::List { .. } => {},
        }
    }

    pub fn unit_type(&self) -> &'static str {
        match self {
            MarkdownUnit::Heading { .. } => "heading",
            MarkdownUnit::Paragraph { .. } => "paragraph",
            MarkdownUnit::CodeBlock { .. } => "codeblock",
            MarkdownUnit::List { .. } => "list",
            MarkdownUnit::BlockQuote { .. } => "blockquote",
            MarkdownUnit::Raw { .. } => "raw",
        }
    }
}

#[derive(Debug, Default)]
pub struct HandlerContext {
    pub line_number: usize,
    pub file_path: Option<String>,
}

pub trait Handler: Send + Sync {
    fn can_handle(&self, unit: &MarkdownUnit) -> bool;
    fn handle(&self, unit: MarkdownUnit, context: &HandlerContext) -> Result<Option<MarkdownUnit>>;
    fn name(&self) -> &str;
}