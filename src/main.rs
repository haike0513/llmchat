use  burn::backend::{self, rocm};

use burn::data::dataset::Dataset;
pub mod model;
pub mod inference;
pub mod data;
pub mod training;
#[tokio::main]
pub async fn main() {
    type MyBackend = backend::Rocm<f32, i32>;
    let device = rocm::HipDevice::new(0);
    println!("Hello, world! {:#?}", device);
    let artifact_dir = "/tmp/guide";

        // Infer the model
        inference::infer::<MyBackend>(
            artifact_dir,
            device,
            burn::data::dataset::vision::MnistDataset::test()
                .get(42)
                .unwrap(),
        );


}
