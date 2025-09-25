
mod api;
mod config;
mod domain;
mod predict;
mod tokenizer;

use actix_cors::Cors;
use actix_files::Files;
use actix_web::{web, App, HttpServer};
use std::sync::Arc;
use tracing_subscriber::EnvFilter;
use tracing::info;

use crate::api::routes;
use crate::config::Config;
use crate::predict::{MockPredictor, Predictor, ProxyPredictor};
use crate::predict::onnx_predictor::OnnxPredictor;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Инициализация логирования
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    info!("Starting Kabanchiki backend server");

    // Загрузка конфигурации
    let config = Config::from_env();
    info!("Configuration loaded: {:?}", config);

    // Инициализация предиктора
    let predictor: web::Data<dyn Predictor> = initialize_predictor(&config).await;

    // Создание HTTP сервера
    let server_host = config.server_host.clone();
    let server_port = config.server_port;
    let static_dir = config.static_dir.clone();

    info!("Starting server on {}:{}", server_host, server_port);

    HttpServer::new(move || {
        let cors = Cors::permissive();
        App::new()
            .wrap(cors)
            .app_data(predictor.clone())
            .configure(routes)
            .service(Files::new("/", &static_dir).index_file("index.html"))
    })
    .bind((server_host.as_str(), server_port))?
    .run()
    .await
}

/// Инициализирует предиктор на основе конфигурации
async fn initialize_predictor(config: &Config) -> web::Data<dyn Predictor> {
    // Приоритет: Proxy -> ONNX -> Mock
    if let Some(proxy_url) = &config.proxy_url {
        info!("Using proxy predictor with URL: {}", proxy_url);
        web::Data::from(Arc::new(ProxyPredictor::new(proxy_url.clone())) as Arc<dyn Predictor>)
    } else {
        let onnx_path = config.model_dir.join("v42_model.onnx");
        if onnx_path.exists() {
            info!("Attempting to initialize ONNX predictor with model: {:?}", onnx_path);
            match OnnxPredictor::try_new(&onnx_path) {
                Ok(predictor) => {
                    info!("ONNX predictor initialized successfully");
                    web::Data::from(Arc::new(predictor) as Arc<dyn Predictor>)
                }
                Err(e) => {
                    eprintln!("Failed to initialize ONNX predictor: {:?}. Falling back to MockPredictor", e);
                    web::Data::from(Arc::new(MockPredictor::new(config.model_dir.clone())) as Arc<dyn Predictor>)
                }
            }
        } else {
            info!("ONNX model not found at {:?}. Using MockPredictor", onnx_path);
            web::Data::from(Arc::new(MockPredictor::new(config.model_dir.clone())) as Arc<dyn Predictor>)
        }
    }
}
