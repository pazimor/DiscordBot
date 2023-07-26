FROM rust:latest
WORKDIR /app
COPY . .
RUN rm -rf target
RUN cargo build --release
CMD ["./discord_bot"]
