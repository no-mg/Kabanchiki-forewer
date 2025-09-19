# Билд стадии
FROM golang:1.25.1-alpine AS builder

RUN apk add --no-cache git ca-certificates

WORKDIR /app

COPY go.mod go.sum ./

RUN go mod download

COPY . .

RUN CGO_ENABLED=0 GOOS=linux go build -o /app/server ./main.go

FROM alpine:3.18

RUN apk add --no-cache ca-certificates

RUN adduser -D -g '' appuser

USER appuser

COPY --from=builder --chown=appuser:appuser /app/server /app/server
COPY --from=builder --chown=appuser:appuser /app/templates /app/templates

WORKDIR /app

EXPOSE 8080

ENTRYPOINT ["/app/server"]