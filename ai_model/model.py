import pandas as pd, joblib, torch
from topic_model_v43_patch import TopicModelV43, Config
import onnx

# ваш файл с отзывами, одна колонка text (UTF-8)
df = pd.read_csv("reviews.csv")
texts = df["text"].astype(str).tolist()

# Обучение модели
mdl = TopicModelV43(Config(verbose=True))
mdl.fit(texts)
mdl.build_terms_and_names(texts)
mdl.calibrate_thresholds(texts[:min(3000, len(texts))])
mdl.build_gold_mapping_and_priors()

# Сохраняем артефакты в pkl
joblib.dump(mdl.export_artifacts(), "v43_artifacts.pkl", compress=3)
print("✅ Артефакты сохранены → v43_artifacts.pkl")

# 🔹 Экспорт в ONNX
import torch.onnx
import numpy as np

class Wrapper(torch.nn.Module):
    def __init__(self, mdl):
        super().__init__()
        self.mdl = mdl
    def forward(self, input_ids, attention_mask):
        # здесь должен быть твой пайплайн
        # для примера просто имитация logits
        logits = torch.randn((input_ids.shape[0], len(mdl.topics)))
        return logits

onnx_model = Wrapper(mdl)

# фиктивный вход (батч из 1, 16 токенов)
dummy_input_ids = torch.randint(0, 1000, (1,16))
dummy_attention_mask = torch.ones_like(dummy_input_ids)

torch.onnx.export(
    onnx_model,
    (dummy_input_ids, dummy_attention_mask),
    "v43_model.onnx",
    input_names=["input_ids", "attention_mask"],
    output_names=["logits"],
    dynamic_axes={
        "input_ids": {0: "batch", 1: "seq"},
        "attention_mask": {0: "batch", 1: "seq"},
        "logits": {0: "batch"}
    },
    opset_version=17
)
print("✅ Экспорт в ONNX завершён → v43_model.onnx")
