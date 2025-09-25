from dataclasses import dataclass
from typing import Any, List
from sklearn.feature_extraction.text import TfidfVectorizer
from sklearn.decomposition import TruncatedSVD


@dataclass
class Config:
    topk_delta_base: int = 3
    topk_delta_long: int = 5
    min_len_for_3: int = 120
    entropy_threshold: float = 0.8
    overlap_min_default: float = 0.2


class TopicModelV42:
    def __init__(self, cfg: Config):
        self.cfg = cfg
        # minimal placeholders for artifacts referenced in model.py
        # Use real sklearn components compatible with skl2onnx
        self.tfidf: Any = TfidfVectorizer(max_features=50000)
        self.svd: Any = None
        self.centroids: Any = []
        self.topic_ids: List[int] = [0]
        self.topic_terms: Any = {0: ["term"]}
        self.topic_names: Any = {0: "Топик"}
        self.topic_sizes: Any = {0: 0}
        self.service_topics: Any = []
        self.mobile_topics: Any = []
        self.overlap_min: float = cfg.overlap_min_default
        self.tau2_global: float = 0.0
        self.tau3_global: float = 0.0
        self.tau2_per: Any = {}
        self.tau3_per: Any = {}
        self.cluster2gold: Any = {}
        self.tag2cluster: Any = {}

    def fit(self, texts: List[str]):
        # Fit TF-IDF then SVD on its output to stay ONNX-compatible
        X = self.tfidf.fit_transform(texts)
        n_features = X.shape[1]
        # Ensure valid n_components for SVD: 1 < n_components <= n_features
        n_components = max(2, min(256, max(2, n_features - 1)))
        self.svd = TruncatedSVD(n_components=n_components, random_state=0)
        self.svd.fit(X)
        return self

    def build_terms_and_names(self, texts: List[str]):
        # no-op stub
        return self

    def calibrate_thresholds(self, texts: List[str]):
        # no-op stub
        return self

    def build_gold_mapping_and_priors(self):
        # no-op stub
        return self


