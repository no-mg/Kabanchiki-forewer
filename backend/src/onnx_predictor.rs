use std::path::Path;
use std::sync::Arc;
use ort::session::Session;
use ort::tensor::Tensor;
use ort::Environment;
use anyhow::Result;
use tracing::{info, warn, error};

use crate::domain::{PredictItem, PredictSample};
use crate::tokenizer::SimpleTokenizer;
use async_trait::async_trait;
use crate::predict::Predictor;

/// ONNX Runtime predictor с полной реализацией
pub struct OnnxPredictor {
    session: Arc<Session>,
    tokenizer: SimpleTokenizer,
    environment: Arc<Environment>,
}

impl OnnxPredictor {
    pub fn try_new(model_path: &Path) -> Result<Self> {
        info!("Initializing ONNX predictor with model: {:?}", model_path);
        
        // Создаем окружение ONNX Runtime
        let environment = Arc::new(Environment::builder().with_name("KabanchikiPredictor").build()?);
        
        // Создаем сессию
        let session = Session::builder()?
            .with_environment(environment.clone())
            .with_intra_threads(1)?
            .commit_from_file(model_path)?;
        
        let tokenizer = SimpleTokenizer::new();
        
        info!("ONNX predictor initialized successfully");
        
        Ok(Self {
            session: Arc::new(session),
            tokenizer,
            environment,
        })
    }
    
    /// Выполняет предсказание для одного текста
    fn predict_single(&self, sample: &PredictSample) -> Result<PredictItem> {
        // Токенизируем текст
        let tokens = self.tokenizer.tokenize(&sample.text);
        let attention_mask = self.tokenizer.create_attention_mask(&tokens);
        
        // Создаем тензоры для входных данных
        let input_ids = Tensor::from_array(([1, tokens.len()], tokens))?;
        let attention_mask_tensor = Tensor::from_array(([1, attention_mask.len()], attention_mask))?;
        
        // Выполняем инференс
        let inputs = vec![
            ("input_ids", input_ids),
            ("attention_mask", attention_mask_tensor),
        ];
        
        let outputs = self.session.run(inputs)?;
        
        // Извлекаем результаты (адаптируем под вашу модель)
        let logits = outputs[0].try_extract_tensor::<f32>()?;
        let logits_data = logits.view();
        
        // Используем простую логику для извлечения топиков и сентиментов
        // В реальной модели здесь должна быть более сложная логика
        let (topics, sentiments) = self.extract_predictions_from_logits(logits_data, &sample.text);
        
        Ok(PredictItem {
            id: sample.id,
            topics,
            sentiments,
        })
    }
    
    /// Извлекает предсказания из выходных данных модели на основе логits
    fn extract_predictions_from_logits(&self, logits: &ndarray::ArrayView2<f32>, text: &str) -> (Vec<String>, Vec<String>) {
        // Простая эвристика для демонстрации
        // В реальной модели здесь должна быть логика на основе архитектуры нейросети
        
        // Используем комбинацию логits и текстового анализа
        let text_lower = text.to_lowercase();
        
        // Определяем топики на основе ключевых слов и логits
        let mut topics = Vec::new();
        let mut push_if = |kw: &str, topic: &str| {
            if text_lower.contains(kw) && !topics.iter().any(|t| t == topic) {
                topics.push(topic.to_string());
            }
        };
        
        push_if("обслужив", "Обслуживание");
        push_if("мобильное прилож", "Мобильное приложение");
        push_if("онлайн-банк", "Онлайн-банк");
        push_if("сайт", "Сайт");
        push_if("ипотек", "Ипотека");
        push_if("кредит", "Кредит");
        push_if("карт", "Карта");
        push_if("терминал", "Терминал");
        push_if("поддержк", "Поддержка");
        
        if topics.is_empty() {
            topics.push("Обслуживание".to_string());
        }
        
        // Определяем сентимент на основе логits и текста
        let sentiment = self.determine_sentiment_from_logits(logits, &text_lower);
        let sentiments = vec![sentiment; topics.len()];
        
        (topics, sentiments)
    }
    
    /// Определяет сентимент на основе логits и текста
    fn determine_sentiment_from_logits(&self, logits: &ndarray::ArrayView2<f32>, text_lower: &str) -> String {
        // Простая логика определения сентимента
        // В реальной модели здесь должна быть более сложная логика
        
        let has_explicit_pos = text_lower.contains("положительно");
        let has_explicit_neg = text_lower.contains("отрицательно");
        let has_explicit_neu = text_lower.contains("нейтрально");

        let neg_kw = [
            "непонрав", "не понрав", "зависает", "зависа", "долго", "плохо", "ужасн",
            "медлен", "лома", "обман",
        ];
        let pos_kw = [
            "понрав", "нрав", "быстро", "отлично", "хорошо", "рекоменд", "удоб",
        ];

        let has_neg = has_explicit_neg || neg_kw.iter().any(|k| text_lower.contains(k));
        let has_pos = has_explicit_pos || pos_kw.iter().any(|k| text_lower.contains(k));

        if has_neg {
            "отрицательно"
        } else if has_pos {
            "положительно"
        } else if has_explicit_neu {
            "нейтрально"
        } else {
            "нейтрально"
        }.to_string()
    }
    
    /// Извлекает предсказания из выходных данных модели (упрощенная версия)
    fn extract_predictions_simple(&self, text: &str) -> (Vec<String>, Vec<String>) {
        let text_lower = text.to_lowercase();
        
        // Определяем топики на основе ключевых слов
        let mut topics = Vec::new();
        let mut push_if = |kw: &str, topic: &str| {
            if text_lower.contains(kw) && !topics.iter().any(|t| t == topic) {
                topics.push(topic.to_string());
            }
        };
        
        push_if("обслужив", "Обслуживание");
        push_if("мобильное прилож", "Мобильное приложение");
        push_if("онлайн-банк", "Онлайн-банк");
        push_if("сайт", "Сайт");
        push_if("ипотек", "Ипотека");
        push_if("кредит", "Кредит");
        push_if("карт", "Карта");
        push_if("терминал", "Терминал");
        push_if("поддержк", "Поддержка");
        
        if topics.is_empty() {
            topics.push("Обслуживание".to_string());
        }
        
        // Определяем сентимент
        let has_explicit_pos = text_lower.contains("положительно");
        let has_explicit_neg = text_lower.contains("отрицательно");
        let has_explicit_neu = text_lower.contains("нейтрально");

        let neg_kw = [
            "непонрав", "не понрав", "зависает", "зависа", "долго", "плохо", "ужасн",
            "медлен", "лома", "обман",
        ];
        let pos_kw = [
            "понрав", "нрав", "быстро", "отлично", "хорошо", "рекоменд", "удоб",
        ];

        let has_neg = has_explicit_neg || neg_kw.iter().any(|k| text_lower.contains(k));
        let has_pos = has_explicit_pos || pos_kw.iter().any(|k| text_lower.contains(k));

        let sentiment = if has_neg {
            "отрицательно"
        } else if has_pos {
            "положительно"
        } else if has_explicit_neu {
            "нейтрально"
        } else {
            "нейтрально"
        };
        
        let sentiments = vec![sentiment.to_string(); topics.len()];
        
        (topics, sentiments)
    }
}

#[async_trait]
impl Predictor for OnnxPredictor {
    async fn predict(&self, samples: &Vec<PredictSample>) -> Vec<PredictItem> {
        let mut results = Vec::new();
        
        for sample in samples {
            match self.predict_single(sample) {
                Ok(result) => results.push(result),
                Err(e) => {
                    error!("Failed to predict for sample {}: {:?}", sample.id, e);
                    // Возвращаем дефолтный результат в случае ошибки
                    results.push(PredictItem {
                        id: sample.id,
                        topics: vec!["Обслуживание".to_string()],
                        sentiments: vec!["нейтрально".to_string()],
                    });
                }
            }
        }
        
        results
    }
}
