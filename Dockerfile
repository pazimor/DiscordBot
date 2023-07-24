FROM rust:latest
WORKDIR /app
COPY . .
RUN rm -rf target
RUN cargo build --release
EXPOSE 8080
CMD ["./discord_bot"]
