use burn::{backend::{self, rocm, wgpu::WgpuRuntime, Autodiff}, optim::decay::WeightDecayConfig};
use clap::Parser;
use llmchat::{DbPediaDataset, training::ExperimentConfig};

pub(crate) mod cache;
pub use llmchat::llama;
pub use llmchat::pretrained;
pub use llmchat::sampling;
pub use llmchat::tokenizer;
pub use llmchat::transformer;
pub use llmchat::chat;
pub use llmchat::router;
pub use llmchat::server;
pub use llmchat::ext;

#[cfg(feature = "f16")]
type Elem = burn::tensor::f16;
#[cfg(not(feature = "f16"))]
type Elem = f32;

type MyBackend = burn::backend::wgpu::CubeBackend<WgpuRuntime, f32, i32, u32>;
type MyAutodiffBackend = Autodiff<MyBackend>;

type TorchBackend = burn::backend::Autodiff<burn::backend::LibTorch<Elem>>;
type Backend = MyAutodiffBackend;
type WGPUBackend = MyAutodiffBackend;
type ROCMBackend = Autodiff<backend::Rocm<f32, i32, u32>>;
fn train_llm_model() {
    let device = rocm::HipDevice::new(0);
    let device = burn::tensor::Device::<TorchBackend>::Cpu;
    let device = burn::tensor::Device::<TorchBackend>::Cpu;
    // let device = backend::wgpu::WgpuDevice::IntegratedGpu(0);
    let device = backend::wgpu::WgpuDevice::default();


    let config = ExperimentConfig::new(
        burn::nn::transformer::TransformerEncoderConfig::new(384, 1536, 12, 6)
            .with_norm_first(true),
        burn::optim::AdamConfig::new().with_weight_decay(Some(WeightDecayConfig::new(1.0e-6))),
    );

    llmchat::training::train::<WGPUBackend, DbPediaDataset>(
        device,
        DbPediaDataset::train(),
        DbPediaDataset::test(),
        config,
        "guide/text-generation",
    );
}
// use chat::chat;

pub mod wgpu {
    use super::*;
    use burn::backend::wgpu::{Wgpu, WgpuDevice};

    pub fn run(args: chat::Config) {
        let device = WgpuDevice::default();

        // chat::chat::<Wgpu>(args, device);
    }
}

// #[cfg(feature = "tch-cpu")]
use chat::Config;
pub mod tch_cpu {
    use std::convert::Infallible;

    use crate::llama::GenerationOutput;

    use super::*;
    use axum::response::sse::Event;
    use burn::backend::{libtorch::LibTorchDevice, LibTorch};
    use tokio::sync::mpsc;

    pub fn run(args: Config, sender: mpsc::Sender<Result<Event, Infallible>>) -> GenerationOutput {
        let device = LibTorchDevice::Cpu;

        chat::chat::<LibTorch>(args, device, sender)
    }
}

use axum::{
    routing::{get, post},
    Router,
};

use std::{collections::BTreeMap, default::Default};
#[tokio::main]
pub async fn main() {
    tracing_subscriber::fmt::init();
    use crate::server::state;
    // let args = chat::Config::parse();
    // wgpu::run(args);
    let app_state = state::AppState{
        ..Default::default()
    };
    // tch_cpu::run(args);
        // build our application with a single route
        let app = Router::new().route("/", get(|| async { "Hello, World!" }))
        .route("/api/v1/chat", post(router::hello_world))
        .route("/api/v1/sse", post(router::sse_handler))
        .with_state(app_state)
        ;

        let listener = tokio::net::TcpListener::bind("0.0.0.0:9320").await.unwrap();
        axum::serve(listener, app).await.unwrap();
}
