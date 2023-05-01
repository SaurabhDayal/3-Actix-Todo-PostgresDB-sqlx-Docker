FROM rust as builder
COPY . /app
WORKDIR /app
RUN cargo build --release


FROM gcr.io/distroless/cc
COPY --from=builder /app/target/release/sqlx-try /app/sqlx-try
WORKDIR /app

CMD ["./sqlx-try"]
EXPOSE 8080
