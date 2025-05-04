#[macro_use]
extern crate derive_new;

mod data;
mod model;
pub mod extensions;
pub mod llm;


pub mod training;
pub use data::DbPediaDataset;
pub mod llama;
pub mod pretrained;
pub mod sampling;
pub mod tokenizer;
pub mod transformer;
pub mod chat;
pub mod router;
pub mod server;
pub mod ext;
pub mod cache;


use chat::Config;
// pub mod tch_cpu {
//     use std::convert::Infallible;

//     use crate::llama::GenerationOutput;

//     use super::*;
//     use axum::response::sse::Event;
//     use burn::backend::{libtorch::LibTorchDevice, LibTorch};
//     use tokio::sync::mpsc;

//     pub async  fn run(args: Config, sender: mpsc::Sender<String>) -> GenerationOutput {
//         let device = LibTorchDevice::Cpu;

//         chat::chat::<LibTorch>("Hello World".to_string(), device, sender).await
//     }
// }

