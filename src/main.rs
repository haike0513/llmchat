use burn::{backend::{self, rocm, wgpu::WgpuRuntime, Autodiff}, optim::decay::WeightDecayConfig};
use clap::Parser;
use llmchat::{DbPediaDataset, training::ExperimentConfig};

pub(crate) mod cache;
pub mod llama;
pub mod pretrained;
pub mod sampling;
pub mod tokenizer;
pub mod transformer;
pub mod chat;

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

mod wgpu {
    use super::*;
    use burn::backend::wgpu::{Wgpu, WgpuDevice};

    pub fn run(args: chat::Config) {
        let device = WgpuDevice::default();

        chat::chat::<Wgpu>(args, device);
    }
}



fn main() {
    let args = chat::Config::parse();
    wgpu::run(args);

}
