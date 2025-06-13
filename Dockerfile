# Etapa 1: Build
FROM rust:1.72 as builder

WORKDIR /usr/src/app

COPY . .

RUN cargo build --release

# Etapa 2: Runtime
FROM debian:buster-slim

RUN apt-get update && apt-get install -y \
    libssl-dev \
    ca-certificates \
    clang \
    musl-tools \
    && rm -rf /var/lib/apt/lists/*


WORKDIR /usr/src/app

# Copia o banco local para o container
COPY data ./data

COPY --from=builder /usr/src/app/target/release/unit_converter_web ./unit_converter_web

EXPOSE 8000

CMD ["./unit_converter_web"]

