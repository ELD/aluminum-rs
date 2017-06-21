FROM bitnami/minideb:jessie

MAINTAINER Eric Dattore

# Install dependencies for kcov
RUN apt-get update
RUN apt-get update && \
    apt-get install -y zlib1g-dev libcurl4-openssl-dev libelf-dev libdw-dev \
    cmake cmake-data g++ binutils-dev libiberty-dev python-minimal git curl unzip pkg-config

# Grab kcov
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

# Install Rust
# TODO: Install nightly and beta
RUN curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain stable -y
RUN echo "source ~/.cargo/env" >> ~/.bashrc
RUN ~/.cargo/bin/rustup install beta
RUN ~/.cargo/bin/rustup install nightly

# Clone Aluminum-rs
RUN git clone https://github.com/ELD/Aluminum-rs /app

# Set workdir
WORKDIR /app
RUN ~/.cargo/bin/rustup run stable cargo fetch
RUN ~/.cargo/bin/rustup run beta cargo fetch
RUN ~/.cargo/bin/rustup run nightly cargo fetch

# Cleanup
RUN rm -rf /app
RUN apt-get clean
