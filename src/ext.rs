use crate::tch_cpu;
use clap::Parser;
use crate::extensions::llm::LLMGenerate;

use futures::{stream::{BoxStream, Stream}, StreamExt};
use tokio_stream::wrappers::ReceiverStream;

use crate::chat;

#[derive(Debug, Clone, Default)]

pub struct LlamaExtension;

impl LlamaExtension {
    pub fn new() -> Self {
        Self {}
    }
    pub fn generate_llm(&self) -> impl Stream<Item = String> + Send + 'static {
        let (tx, mut rx) = tokio::sync::mpsc::channel::<String>(100);

        let instance = self.clone();

        tokio::spawn(async move {
            instance.generate_with_prompt("hello".to_string(), tx).await;
        });

        let rs = ReceiverStream::new(rx);
        rs
    }
    pub async fn generate_with_prompt(&self, prompt: String, tx: tokio::sync::mpsc::Sender<String>) {
        tx.send("value".to_string()).await.unwrap();

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
