FROM rust:1.46

WORKDIR /app

RUN apt-get update && apt-get -y install libopencv-dev clang 
RUN apt-get -y install libclang-dev

COPY . /app/

RUN cargo build

CMD cargo test