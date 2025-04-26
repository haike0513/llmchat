use  burn::backend::{self, rocm, Autodiff};

use burn::data::dataset::Dataset;
use burn::optim::AdamConfig;
use model::ModelConfig;
use training::TrainingConfig;
pub mod model;
pub mod inference;
pub mod data;
pub mod training;
// #[tokio::main]
pub  fn main() {
    type MyBackend = backend::Rocm<f32, i32>;
    let device = rocm::HipDevice::new(0);
    println!("Hello, world! {:#?}", device);

    type MyAutodiffBackend = Autodiff<MyBackend>;

    // All the training artifacts will be saved in this directory
    let artifact_dir = "/tmp/guide";

    // Train the model
    training::train::<MyAutodiffBackend>(
        artifact_dir,
        TrainingConfig::new(ModelConfig::new(10, 512), AdamConfig::new()),
        device.clone(),
    );

        // Infer the model
        inference::infer::<MyBackend>(
            artifact_dir,
            device,
            burn::data::dataset::vision::MnistDataset::test()
                .get(42)
                .unwrap(),
        );


}
