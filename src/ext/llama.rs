use tokio::sync::mpsc;

use std::convert::Infallible;

use super::*;
use axum::response::sse::Event;
use burn::backend::{libtorch::LibTorchDevice, LibTorch};
use burn::tensor::{backend::Backend, Device};

pub async fn run(
    sender: mpsc::Sender<String>,
    prompt: String,
) {
    let device = LibTorchDevice::Cpu;
    // chat(device, prompt, sender);
    crate::chat::chat::<LibTorch>(prompt, device, sender).await;

}

// pub async fn chat<B: Backend>(
//     device: Device<B>,
//     prompt: String,
//     sender: mpsc::Sender<String>,
// ) -> GenerationOutput {
//     let mut sampler = Sampler::Argmax;

//     let mut llama = LlamaConfig::tiny_llama_pretrained::<B>(1024, &device).unwrap();

    

// }

use crate::{
    llama::{GenerationOutput, Llama, LlamaConfig},
    sampling::{Sampler, TopP},
    tokenizer::Tokenizer,
};

// pub  fn generate<B: Backend, T: Tokenizer>(
//     llama: &mut Llama<B, T>,
//     prompt: &str,
//     sample_len: usize,
//     temperature: f64,
//     sampler: &mut Sampler,
//     sender: mpsc::Sender<String>,
// )  {
//     // 
    
// }