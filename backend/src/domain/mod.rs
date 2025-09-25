use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Topic { pub id: i32, pub name: String }

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SentimentStats { pub positive: i64, pub neutral: i64, pub negative: i64 }

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TopicsStatsItem { pub id: i32, pub name: String, pub stats: SentimentStats }

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Period { pub from: String, pub to: String }

#[derive(Debug, Serialize, Deserialize)]
pub struct TopicsStatsResponse { pub period: Period, pub topics: Vec<TopicsStatsItem> }

#[derive(Debug, Serialize, Deserialize)]
pub struct TimelinePoint { pub date: String, pub positive: i64, pub neutral: i64, pub negative: i64 }

#[derive(Debug, Serialize, Deserialize)]
pub struct TimelineResponse { pub topic: Topic, pub timeline: Vec<TimelinePoint> }

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[allow(dead_code)]
pub enum Sentiment { Positive, Neutral, Negative }

#[derive(Debug, Serialize, Deserialize)]
pub struct ReviewItem { pub id: i64, pub date: String, pub sentiment: String, pub text: String, pub region: String }

#[derive(Debug, Serialize, Deserialize)]
pub struct ReviewsFilters { pub topic_id: Option<i32>, pub sentiment: Option<String>, pub period: Option<Period> }

#[derive(Debug, Serialize, Deserialize)]
pub struct Pagination { pub page: i64, pub limit: i64, pub total: i64 }

#[derive(Debug, Serialize, Deserialize)]
pub struct ReviewsResponse { pub filters: ReviewsFilters, pub pagination: Pagination, pub reviews: Vec<ReviewItem> }

#[derive(Debug, Deserialize)]
pub struct StatsQuery { pub date_from: String, pub date_to: String, pub _region: Option<String> }

#[derive(Debug, Deserialize)]
pub struct TimelineQuery { pub _date_from: String, pub _date_to: String, pub group_by: String, pub _region: Option<String> }

#[derive(Debug, Deserialize)]
pub struct ReviewsQuery {
    pub topic_id: Option<i32>,
    pub date_from: Option<String>,
    pub date_to: Option<String>,
    pub sentiment: Option<String>,
    pub _region: Option<String>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct PredictRequest { pub data: Vec<PredictSample> }

#[derive(Debug, Serialize, Deserialize)]
pub struct PredictSample { pub id: i64, pub text: String }

#[derive(Debug, Serialize)]
pub struct PredictResponse { pub predictions: Vec<PredictItem> }

#[derive(Debug, Serialize)]
pub struct PredictItem { pub id: i64, pub topics: Vec<String>, pub sentiments: Vec<String> }


