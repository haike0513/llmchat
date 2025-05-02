use crate::tch_cpu;
use clap::Parser;
use llmchat::extensions::llm::LLMGenerate;

use futures::{stream::{BoxStream, Stream}, StreamExt};
use tokio_stream::wrappers::ReceiverStream;

use crate::chat;

#[derive(Debug, Clone, Default)]

pub struct LlamaExtension;

impl LlamaExtension {
    pub fn new() -> Self {
        Self {}
    }
    pub fn generate_llm(&self) -> impl Stream<Item = String> + 'static {
        let (tx, mut rx) = tokio::sync::mpsc::channel::<String>(100);

        tokio::spawn(async move {
            tx.send("Stream".to_string()).await.unwrap();
            tx.closed();
        });

        let rs = ReceiverStream::new(rx);
        rs
    }
}

impl LLMGenerate for LlamaExtension {
    fn name(&self) -> &'static str {
        "Llama"
    }

    fn generate(&self) -> futures::stream::BoxStream<'static, String> {
        self.generate_llm().boxed()
        // let (tx, rx) = tokio::sync::mpsc::channel(32);

        // let args = chat::Config::parse();
        // let g = tch_cpu::run(args, tx);
        // let stream = tokio_stream::wrappers::ReceiverStream::new(rx);
        // let ss = stream.map(|s|{
        //     let a = s.unwrap();
        //     "".to_string()
        // }).boxed();
        // ss
    }
}
