FROM rust:1.46

WORKDIR /app

RUN apt-get update && \
    DEBIAN_FRONTEND=noninteracti apt-get -y install libclang-dev clang curl git zip unzip pkg-config

ENV VCPKG_ROOT /usr/src/vcpkg
ENV VCPKGRS_DYNAMIC=1
ENV PATH "/bin:/sbin:/usr/bin:/usr/sbin:/root/.cargo/bin"

RUN bash -exc 'cd /usr/src && git clone https://github.com/microsoft/vcpkg && cd vcpkg && ./bootstrap-vcpkg.sh && ./vcpkg install opencv4[contrib,nonfree] && ./vcpkg list'
