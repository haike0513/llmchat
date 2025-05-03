use std::sync::Arc;

use crate::tch_cpu;
use crate::{extensions::llm::LLMGenerate, tokenizer::SentiencePieceTokenizer};
use burn::backend::LibTorch;
use clap::Parser;
use tokio::sync::Mutex;
use tokio::{pin, sync::mpsc};

use futures::{
    StreamExt,
    stream::{BoxStream, Stream, unfold},
};
use tokio_stream::wrappers::ReceiverStream;

use crate::chat;

pub mod llama;

use crate::llama::{Llama, LlamaConfig};

#[derive()]
pub struct LlamaModel {
    pub llama: Option<Llama<LibTorch, SentiencePieceTokenizer>>,
}

unsafe impl Sync for LlamaModel {}

unsafe impl Send for LlamaModel {}
use crate::sampling::{Sampler, TopP};

impl LlamaModel {
    pub async fn generate(&mut self, prompt: String, tx: mpsc::Sender<String>) {
        // llama::run(tx, prompt).await;
        let mut sampler = Sampler::TopP(TopP::new(0.9, 42));

        let mut llama = self.llama.as_mut().unwrap();
        crate::chat::generate(
            llama,
            prompt.as_str(),
            65,
            0.6,
            &mut sampler,
            tx,
        )
        .await;
    }
    pub async fn load_model(&mut self) {
        let device = burn::tensor::Device::<LibTorch>::Cpu;
        let model = LlamaConfig::tiny_llama_pretrained::<LibTorch>(1024, &device).unwrap();
        self.llama = Some(model);
    }
}

#[derive(Clone)]

pub struct LlamaExtension {
    model: Arc<Mutex<LlamaModel>>,
}

impl LlamaExtension {
    pub fn new() -> Self {
        let model = LlamaModel { llama: None };
        Self {
            model: Arc::new(Mutex::new(model)),
        }
    }
    pub async fn generate_token(&self, prompt: String, tx: mpsc::Sender<String>) {
        let mut model = self.model.lock().await;
        if model.llama.is_none() {
            model.load_model().await;
        }
        model.generate(prompt, tx).await;
        // model.generate(prompt, tx).await;
        // let mut instance = self.model.lock().await;
        // if instance.llama.is_none() {
        //     instance.load_model().await;
        // }
        // instance.generate(prompt, tx).await;
    }

    // pub fn generate_llm(&self, prompt: String) -> impl Stream<Item = String> + Send + 'static {
    //     let (tx, mut rx) = tokio::sync::mpsc::channel::<String>(100);

    //     let instance = self.model.clone();

    //     tokio::spawn(async move {
    //         // let mut instance = instance.lock().await;
    //         // if instance.llama.is_none() {
    //         //     instance.load_model().await;
    //         // }
    //         let prompt = prompt.clone();
    //         instance.lock().await.generate(prompt.clone(), tx).await;
    //     });

    //     // use async_stream::stream;

    //     // let s = stream! {
    //     //     while let Some(value) =  rx.recv().await {
    //     //         yield value
    //     //     }
    //     // };

    //     // // pin!(s);
    //     // s

    //     let rs = ReceiverStream::new(rx);
    //     rs
    // }
}

impl LLMGenerate for LlamaExtension {
    fn name(&self) -> &'static str {
        "Llama"
    }

    fn generate(&self, prompt: String) -> futures::stream::BoxStream<'static, String> {
        let (tx, mut rx) = tokio::sync::mpsc::channel::<String>(100);
        let instance = self.clone();
        tokio::spawn(async move {
            // let mut instance = instance.lock().await;
            // if instance.llama.is_none() {
            //     instance.load_model().await;
            // }
            let prompt = prompt.clone();
            instance.generate_token(prompt, tx).await;
            // instance.lock().await.generate(prompt.clone(), tx).await;
        });
        // self.generate_llm(prompt).boxed()
        ReceiverStream::new(rx).boxed()
    }
}
