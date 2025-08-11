FROM rust:1.86 AS chef
RUN cargo install cargo-chef
WORKDIR /workspace

FROM chef AS planner
COPY . .
RUN ls -laR /workspace
RUN cd /workspace/app && cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /workspace/app/recipe.json recipe.json
RUN ls -laR /workspace
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .

RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
RUN cargo binstall dioxus-cli --root /.cargo -y --force
ENV PATH="/.cargo/bin:$PATH"

RUN cd /workspace/app/ && dx bundle --platform web

FROM chef AS runtime
COPY --from=builder /workspace/app/target/dx/voxfila/release/web/ /usr/local/app

ENV PORT=8080
ENV IP=0.0.0.0

EXPOSE 8080

WORKDIR /usr/local/app
ENTRYPOINT [ "/usr/local/app/server" ]