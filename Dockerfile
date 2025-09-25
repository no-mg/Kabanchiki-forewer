FROM rust:1.80 as builder
WORKDIR /app

# Копируем файлы зависимостей
COPY backend/Cargo.toml backend/Cargo.lock ./
RUN mkdir -p src && echo "fn main(){}" > src/main.rs

# Собираем зависимости (кэшируем этот слой)
RUN cargo build --release || true

# Копируем исходный код
COPY backend/src ./src
COPY backend/Cargo.toml backend/Cargo.lock ./

# Собираем приложение
RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /app

# Устанавливаем необходимые пакеты
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Копируем собранное приложение
COPY --from=builder /app/target/release/backend /usr/local/bin/backend

# Копируем статические файлы и AI модель
COPY backend/frontend ./frontend
COPY ai_model ./ai_model

# Настраиваем переменные окружения
ENV RUST_LOG=info
ENV MODEL_DIR=/app/ai_model
ENV SERVER_HOST=0.0.0.0
ENV SERVER_PORT=8080

EXPOSE 8080
CMD ["/usr/local/bin/backend"]

