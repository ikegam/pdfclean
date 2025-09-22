pub mod traits;
pub mod whitespace_processor;
pub mod image_processor;
pub mod table_processor;

pub use traits::{NodeProcessor, ProcessContext};
pub use whitespace_processor::WhitespaceProcessor;
pub use image_processor::ImageProcessor;
pub use table_processor::TableProcessor;