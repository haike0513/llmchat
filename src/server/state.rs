use std::{collections::BTreeMap, sync::{Arc, Mutex}};

use llmchat::extensions::llm::LLMGenerate;
use crate::ext::LlamaExtension;

#[derive(Clone)]
pub struct AppState {
    pub model: String,
    pub tokenizer: String,
    pub device: String,
    pub version: String,
    pub models: Arc<Mutex<BTreeMap<String, Box<dyn LLMGenerate>>>>,
}
impl AppState {
    pub fn new(model: String, tokenizer: String, device: String, version: String) -> Self {
        Self {
            ..Default::default()
        }
    }
}

impl Default for  AppState {
    fn default() -> Self {
        let mut models: Arc<Mutex<BTreeMap<String, Box<dyn LLMGenerate>>>> = Arc::new(Mutex::new(BTreeMap::default()));
        let ex = LlamaExtension;
        models.lock().unwrap().insert("llm".to_string(), Box::new(ex));
        Self { model: Default::default(), tokenizer: Default::default(), device: Default::default(), version: Default::default(), models }
    }
}