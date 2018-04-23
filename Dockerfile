FROM linkerd/rustup-nightly:latest

RUN /install-rust.sh

COPY . /app
WORKDIR /app
RUN cargo build --release

