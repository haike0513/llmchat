use futures::Stream;
use futures::stream::{BoxStream, StreamExt};

pub trait LLMGenerate: Send + Sync {
    fn name(&self) -> &'static str;
    fn generate(&self, prompt: String) -> BoxStream<'static, String>;
}