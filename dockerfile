FROM rust as builder
WORKDIR /usr/src/app/
COPY . .
RUN cargo install --features telegram_notifier --path .

FROM debian:bullseye-slim
COPY --from=builder /usr/local/cargo/bin/dockserver /usr/local/bin/dockserver
ENTRYPOINT ["dockserver"]
