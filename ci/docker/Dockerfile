FROM bitnami/minideb:jessie

MAINTAINER Eric Dattore

RUN apt-get update
RUN apt-get update && \
    apt-get install -y zlib1g-dev libcurl4-openssl-dev libelf-dev libdw-dev \
    cmake cmake-data g++ binutils-dev libiberty-dev python-minimal git curl unzip pkg-config

RUN curl -LOko master.zip https://github.com/SimonKagstrom/kcov/archive/master.zip \
    && unzip master.zip \
    && rm master.zip \
    && cd kcov-master \
    && mkdir build \
    && cd build \
    && cmake .. \
    && make -j4 \
    && make install \
    && cd / \
    && rm -rf kcov-master

RUN curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain stable -y
RUN echo "source ~/.cargo/env" >> ~/.bashrc

RUN git clone https://github.com/ELD/Aluminum-rs /app

WORKDIR /app

RUN ~/.cargo/bin/cargo fetch

RUN apt-get clean

ENTRYPOINT ["/bin/bash"]
