use std::path::PathBuf;
use crate::domain::{PredictItem, PredictSample};
use async_trait::async_trait;

pub mod onnx_predictor;

#[async_trait]
pub trait Predictor: Send + Sync {
    async fn predict(&self, samples: &Vec<PredictSample>) -> Vec<PredictItem>;
}

/// Mock predictor для тестирования и fallback
pub struct MockPredictor {
    _model_dir: PathBuf,
}

impl MockPredictor {
    pub fn new(model_dir: PathBuf) -> Self { 
        Self { _model_dir: model_dir } 
    }
}

#[async_trait]
impl Predictor for MockPredictor {
    async fn predict(&self, samples: &Vec<PredictSample>) -> Vec<PredictItem> {
        samples
            .iter()
            .map(|s| {
                let text_l = s.text.to_lowercase();

                // Topics by simple keyword matching
                let mut topics: Vec<String> = Vec::new();
                let mut push_if = |kw: &str, topic: &str| {
                    if text_l.contains(kw) && !topics.iter().any(|t| t == topic) {
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

                // Sentiment detection (robust keyword sets, negative has precedence)
                let has_explicit_pos = text_l.contains("положительно");
                let has_explicit_neg = text_l.contains("отрицательно");
                let has_explicit_neu = text_l.contains("нейтрально");

                let neg_kw = [
                    "непонрав", "не понрав", "зависает", "зависа", "долго", "плохо", "ужасн",
                    "медлен", "лома", "обман",
                ];
                let pos_kw = [
                    "понрав", "нрав", "быстро", "отлично", "хорошо", "рекоменд", "удоб",
                ];

                let has_neg = has_explicit_neg || neg_kw.iter().any(|k| text_l.contains(k));
                let has_pos = has_explicit_pos || pos_kw.iter().any(|k| text_l.contains(k));

                let sentiment = if has_neg {
                    "отрицательно"
                } else if has_pos {
                    "положительно"
                } else if has_explicit_neu {
                    "нейтрально"
                } else {
                    "нейтрально"
                };

                PredictItem {
                    id: s.id,
                    topics: topics.clone(),
                    sentiments: vec![sentiment.to_string(); topics.len()],
                }
            })
            .collect()
    }
}

// Proxy predictor calls external Python service compatible with our /predict schema
pub struct ProxyPredictor { client: reqwest::Client, url: String }

impl ProxyPredictor {
    pub fn new(url: String) -> Self { Self { client: reqwest::Client::new(), url } }
}

#[async_trait]
impl Predictor for ProxyPredictor {
    async fn predict(&self, samples: &Vec<PredictSample>) -> Vec<PredictItem> {
        let body = serde_json::json!({ "data": samples });
        match self.client.post(&self.url).json(&body).send().await {
            Ok(resp) => match resp.json::<serde_json::Value>().await {
                Ok(json) => json
                    .get("predictions")
                    .and_then(|v| v.as_array())
                    .map(|arr| {
                        arr.iter()
                            .filter_map(|it| {
                                Some(PredictItem {
                                    id: it.get("id")?.as_i64()?,
                                    topics: it.get("topics")?.as_array()?.iter().filter_map(|t| t.as_str().map(|s| s.to_string())).collect(),
                                    sentiments: it.get("sentiments")?.as_array()?.iter().filter_map(|t| t.as_str().map(|s| s.to_string())).collect(),
                                })
                            })
                            .collect::<Vec<_>>()
                    })
                    .unwrap_or_default(),
                Err(_) => vec![],
            },
            Err(_) => vec![],
        }
    }
}


