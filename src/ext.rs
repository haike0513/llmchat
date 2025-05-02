use std::sync::Arc;

use crate::extensions::llm::LLMGenerate;
use crate::tch_cpu;
use clap::Parser;
use tokio::sync::mpsc;

use futures::{
    StreamExt,
    stream::{BoxStream, Stream},
};
use tokio_stream::wrappers::ReceiverStream;

use crate::chat;

pub mod llama;

#[derive(Clone, Debug)]
pub struct LlamaModel {}

impl LlamaModel {
    pub async fn generate(&self, prompt: String, tx: mpsc::Sender<String>) {
        llama::run(tx, prompt).await;
    }
}

#[derive(Debug, Clone)]

pub struct LlamaExtension {
    model: Arc<LlamaModel>,
}

impl LlamaExtension {
    pub fn new() -> Self {
        let model = LlamaModel {};
        Self {
            model: Arc::new(model),
        }
    }
    pub fn generate_llm(&self, prompt: String) -> impl Stream<Item = String> + Send + 'static {
        let (tx, mut rx) = tokio::sync::mpsc::channel::<String>(100);

        let instance = self.model.clone();

        tokio::spawn(async move {
            instance.generate(prompt.clone(), tx).await;
            // instance.generate_with_prompt(prompt, tx).await;
        });

        let rs = ReceiverStream::new(rx);
        rs
    }
    pub async fn generate_with_prompt(
        &self,
        prompt: String,
        tx: tokio::sync::mpsc::Sender<String>,
    ) {
        self.model.generate(prompt, tx).await;
    }
}

impl LLMGenerate for LlamaExtension {
    fn name(&self) -> &'static str {
        "Llama"
    }

    fn generate(&self, prompt: String) -> futures::stream::BoxStream<'static, String> {
        self.generate_llm(prompt).boxed()
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
