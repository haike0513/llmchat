#[derive(Debug, Clone, Default)]
pub struct AppState {
    pub model: String,
    pub tokenizer: String,
    pub device: String,
    pub version: String,
}
impl AppState {
    pub fn new(model: String, tokenizer: String, device: String, version: String) -> Self {
        Self {
            model,
            tokenizer,
            device,
            version,
        }
    }
}