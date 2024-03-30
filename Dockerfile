FROM rust:buster AS chef
RUN cargo install cargo-chef
WORKDIR app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release

FROM debian:buster-slim
EXPOSE 8000
COPY --from=builder /app/target/release/abcd-layered-architecture /abcd-layered-architecture
CMD [ "/abcd-layered-architecture" ]