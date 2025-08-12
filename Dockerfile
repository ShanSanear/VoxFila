FROM rust:1.88.0-slim AS chef
RUN cargo install cargo-chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json



FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .

RUN apt update && apt install -y curl
RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
ENV PATH="/.cargo/bin:$PATH"
RUN cargo binstall dioxus-cli --root /.cargo -y --force


RUN dx bundle --platform web -p app

RUN strip /app/target/dx/app/release/web/server

FROM debian:bullseye-slim AS runtime
COPY --from=builder /app/target/dx/app/release/web/ /usr/local/app

ENV PORT=8080
ENV IP=0.0.0.0

EXPOSE 8080

WORKDIR /usr/local/app
ENTRYPOINT [ "/usr/local/app/server" ]