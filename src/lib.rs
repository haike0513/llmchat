#[macro_use]
extern crate derive_new;

mod data;
mod model;
pub mod extensions;


pub mod training;
pub use data::DbPediaDataset;
