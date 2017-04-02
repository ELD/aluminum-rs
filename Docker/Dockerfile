FROM ubuntu:16.04

MAINTAINER Eric Dattore

RUN apt-get update
RUN apt-get install -y --fix-missing pkg-config
RUN apt-get install -y apt-utils
RUN apt-get install -y zlib1g wget libcurl4-openssl-dev libelf-dev libdw-dev cmake cmake-data g++ binutils-dev \
                                              libiberty-dev zlib1g-dev python-minimal git curl zsh unzip
RUN curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain nightly -y
RUN cd /home && wget https://github.com/SimonKagstrom/kcov/archive/master.zip && unzip master.zip
RUN cd /home/kcov-master && mkdir build && cd build && cmake .. && make -j2 && make install

ENTRYPOINT ["/bin/zsh"]
