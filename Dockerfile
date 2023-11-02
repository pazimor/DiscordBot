FROM rust:latest
WORKDIR /app
COPY . /app/
RUN rm -rf target
RUN cargo build --release
RUN cp ./target/release/discord_bot .
ENV DISCORD_TOKEN=*TOKEN*
CMD ["./target/release/discord_bot"]
