from fastapi import FastAPI
from pydantic import BaseModel
from typing import List

from simple_predict import predict_texts


class PredictSample(BaseModel):
    id: int
    text: str


class PredictRequest(BaseModel):
    data: List[PredictSample]


app = FastAPI()


@app.post("/predict")
def predict(req: PredictRequest):
    items = [s.model_dump() for s in req.data]
    return predict_texts(items)


