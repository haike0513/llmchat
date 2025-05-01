use crate::tch_cpu;
use clap::Parser;
use llmchat::extensions::llm::LLMGenerate;

use futures::{stream::{BoxStream, Stream}, StreamExt};

use crate::chat;

#[derive(Debug, Clone, Default)]

pub struct LlamaExtension;

impl LLMGenerate for LlamaExtension {
    fn name(&self) -> &'static str {
        "Llama"
    }

    fn generate(&self) -> futures::stream::BoxStream<'static, String> {
        let (tx, rx) = tokio::sync::mpsc::channel(32);

        let args = chat::Config::parse();
        let g = tch_cpu::run(args, tx);
        let stream = tokio_stream::wrappers::ReceiverStream::new(rx);
        let ss = stream.map(|s|{
            let a = s.unwrap();
            "".to_string()
        }).boxed();
        ss
    }
}
