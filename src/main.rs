use burn::{backend::{self, rocm, Autodiff}, optim::decay::WeightDecayConfig};
use llmchat::{DbPediaDataset, training::ExperimentConfig};

#[cfg(feature = "f16")]
type Elem = burn::tensor::f16;
#[cfg(not(feature = "f16"))]
type Elem = f32;

type MyBackend = backend::Rocm<f32, i32>;
type MyAutodiffBackend = Autodiff<MyBackend>;

type TorchBackend = burn::backend::Autodiff<burn::backend::LibTorch<Elem>>;
type Backend = MyAutodiffBackend;


fn main() {

    let device = rocm::HipDevice::new(0);
    let device = burn::tensor::Device::<TorchBackend>::Cpu;
    let config = ExperimentConfig::new(
        burn::nn::transformer::TransformerEncoderConfig::new(384, 1536, 12, 6)
            .with_norm_first(true),
        burn::optim::AdamConfig::new().with_weight_decay(Some(WeightDecayConfig::new(1.0e-6))),
    );

    llmchat::training::train::<TorchBackend, DbPediaDataset>(
        device,
        DbPediaDataset::train(),
        DbPediaDataset::test(),
        config,
        "guide/text-generation",
    );
}
