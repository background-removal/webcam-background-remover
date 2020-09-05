FROM ubuntu:20.04

WORKDIR /app

RUN apt-get update && \
    DEBIAN_FRONTEND=noninteracti apt-get -y install libopencv-dev libclang-dev clang curl && \
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

ENV PATH "/bin:/sbin:/usr/bin:/usr/sbin:/root/.cargo/bin"

COPY Cargo.* ./
COPY background-removal background-removal/
COPY gstreamer gstreamer/

RUN cargo build

CMD cargo test