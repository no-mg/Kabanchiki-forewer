#!/usr/bin/env python3
import json
import sys
from typing import List, Dict, Any


def predict_texts(items: List[Dict[str, Any]]) -> Dict[str, Any]:
    def classify(text: str) -> Dict[str, Any]:
        tl = text.lower()

        topics = []
        def push_if(kw: str, topic: str):
            if kw in tl and topic not in topics:
                topics.append(topic)

        push_if("обслужив", "Обслуживание")
        push_if("мобильное прилож", "Мобильное приложение")
        push_if("онлайн-банк", "Онлайн-банк")
        push_if("сайт", "Сайт")
        push_if("ипотек", "Ипотека")
        push_if("кредит", "Кредит")
        push_if("карт", "Карта")
        push_if("терминал", "Терминал")
        push_if("поддержк", "Поддержка")

        if not topics:
            topics.append("Обслуживание")

        has_explicit_pos = "положительно" in tl
        has_explicit_neg = "отрицательно" in tl
        has_explicit_neu = "нейтрально" in tl

        neg_kw = [
            "непонрав", "не понрав", "зависает", "зависа", "долго", "плохо", "ужасн",
            "медлен", "лома", "обман",
        ]
        pos_kw = [
            "понрав", "нрав", "быстро", "отлично", "хорошо", "рекоменд", "удоб",
        ]

        has_neg = has_explicit_neg or any(k in tl for k in neg_kw)
        has_pos = has_explicit_pos or any(k in tl for k in pos_kw)

        if has_neg:
            sent = "отрицательно"
        elif has_pos:
            sent = "положительно"
        elif has_explicit_neu:
            sent = "нейтрально"
        else:
            sent = "нейтрально"

        return {
            "topics": topics,
            "sentiments": [sent for _ in topics],
        }

    preds = []
    for it in items:
        cid = int(it.get("id"))
        text = str(it.get("text", ""))
        cls = classify(text)
        preds.append({
            "id": cid,
            "topics": cls["topics"],
            "sentiments": cls["sentiments"],
        })

    return {"predictions": preds}


if __name__ == "__main__":
    # Usage 1: echo '{"data":[{"id":1,"text":"..."}]}' | python simple_predict.py
    # Usage 2: python simple_predict.py --text "Очень понравилось обслуживание" --id 1
    import argparse
    parser = argparse.ArgumentParser()
    parser.add_argument("--text", type=str, default=None)
    parser.add_argument("--id", type=int, default=1)
    args = parser.parse_args()

    if args.text is not None:
        data = {"data": [{"id": args.id, "text": args.text}]}
    else:
        data = json.load(sys.stdin)

    out = predict_texts(data.get("data", []))
    print(json.dumps(out, ensure_ascii=False))


