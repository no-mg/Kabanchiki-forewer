use actix_web::{get, post, web, Responder};

use crate::domain::*;
use crate::predict::Predictor;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_topics)
        .service(get_topics_stats)
        .service(get_topic_timeline)
        .service(get_reviews)
        .service(post_predict);
}

#[get("/topics")]
async fn get_topics() -> impl Responder {
    let topics = vec![
        Topic { id: 1, name: "Ипотека".into() },
        Topic { id: 2, name: "Карты".into() },
        Topic { id: 3, name: "Кредиты".into() },
        Topic { id: 4, name: "Вклады".into() },
    ];
    web::Json(serde_json::json!({ "topics": topics }))
}

#[get("/topics/stats")]
async fn get_topics_stats(query: web::Query<StatsQuery>) -> impl Responder {
    let period = Period { from: query.date_from.clone(), to: query.date_to.clone() };
    let topics = vec![
        TopicsStatsItem { id: 1, name: "Ипотека".into(), stats: SentimentStats { positive: 120, neutral: 45, negative: 35 }},
        TopicsStatsItem { id: 2, name: "Карты".into(), stats: SentimentStats { positive: 200, neutral: 80, negative: 60 }},
    ];
    web::Json(TopicsStatsResponse { period, topics })
}

#[get("/topics/{topic_id}/timeline")]
async fn get_topic_timeline(path: web::Path<i32>, query: web::Query<TimelineQuery>) -> impl Responder {
    let topic_id = path.into_inner();
    let topic = match topic_id {
        1 => Topic { id: 1, name: "Ипотека".into() },
        2 => Topic { id: 2, name: "Карты".into() },
        3 => Topic { id: 3, name: "Кредиты".into() },
        4 => Topic { id: 4, name: "Вклады".into() },
        _ => Topic { id: topic_id, name: format!("Топик {}", topic_id) },
    };
    let _group_by = &query.group_by;
    let timeline = vec![
        TimelinePoint { date: "2024-01-01".into(), positive: 5, neutral: 2, negative: 1 },
        TimelinePoint { date: "2024-01-02".into(), positive: 8, neutral: 3, negative: 4 },
        TimelinePoint { date: "2024-01-03".into(), positive: 12, neutral: 5, negative: 2 },
    ];
    web::Json(TimelineResponse { topic, timeline })
}

#[get("/reviews")]
async fn get_reviews(query: web::Query<ReviewsQuery>) -> impl Responder {
    let period = match (query.date_from.clone(), query.date_to.clone()) {
        (Some(from), Some(to)) => Some(Period { from, to }),
        _ => None,
    };
    let filters = ReviewsFilters { topic_id: query.topic_id, sentiment: query.sentiment.clone(), period };
    let pagination = Pagination { page: query.page.unwrap_or(1), limit: query.limit.unwrap_or(20), total: 247 };
    let reviews = vec![
        ReviewItem { id: 9321, date: "2024-01-03".into(), sentiment: "negative".into(), text: "Очень долго оформляется ипотека!".into(), region: "Москва".into() },
        ReviewItem { id: 9322, date: "2024-01-03".into(), sentiment: "negative".into(), text: "Банк затянул с одобрением заявки.".into(), region: "Санкт-Петербург".into() },
    ];
    web::Json(ReviewsResponse { filters, pagination, reviews })
}

#[post("/predict")]
async fn post_predict(predictor: web::Data<dyn Predictor>, payload: web::Json<PredictRequest>) -> impl Responder {
    let preds = predictor.predict(&payload.data).await;
    web::Json(PredictResponse { predictions: preds })
}

