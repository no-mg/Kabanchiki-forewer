use std::path::PathBuf;
use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub server_host: String,
    pub server_port: u16,
    pub model_dir: PathBuf,
    pub proxy_url: Option<String>,
    pub static_dir: PathBuf,
}

impl Config {
    pub fn from_env() -> Self {
        let server_host = env::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
        let server_port = env::var("SERVER_PORT")
            .unwrap_or_else(|_| "8080".to_string())
            .parse()
            .unwrap_or(8080);
        
        let model_dir_path = env::var("MODEL_DIR").unwrap_or_else(|_| "ai_model".to_string());
        let model_dir = std::env::current_dir().unwrap().join(model_dir_path);
        
        let proxy_url = env::var("PREDICT_URL").ok().filter(|s| !s.trim().is_empty());
        
        let static_dir = std::env::current_dir().unwrap().join("frontend");

        Self {
            server_host,
            server_port,
            model_dir,
            proxy_url,
            static_dir,
        }
    }
}
