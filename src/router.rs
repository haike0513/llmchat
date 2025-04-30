use axum::{
    routing::{get, post},
    Router,
};
use clap::Parser;
use crate::chat;
use crate::tch_cpu;
pub async fn hello_world() -> String {
    let args = chat::Config::parse();
    let g = tch_cpu::run(args);
    g.text
}