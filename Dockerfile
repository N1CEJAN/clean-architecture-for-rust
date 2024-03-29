FROM rust:buster AS base
WORKDIR /app
RUN cargo init
COPY Cargo.toml .
RUN cargo fetch
COPY . .

FROM base AS development
EXPOSE 8000
CMD [ "cargo", "run", "--offline" ]

FROM base AS builder
RUN cargo build --release --offline

FROM debian:buster-slim as production
EXPOSE 8000
COPY --from=builder /app/target/release/abcd-layered-architecture /abcd-layered-architecture
CMD [ "/abcd-layered-architecture" ]