#@IgnoreInspection BashAddShebang
if [[ $TRAVIS_RUST_VERSION = 'stable' ]]; then
    # FIXME: Dirty hack to get kcov working for now...
    wget https://github.com/SimonKagstrom/kcov/archive/v33.zip
    unzip v33.zip
    cd kcov-33
    mkdir build
    cd build
    cmake ..
    make
    make install DESTDIR=../../tmp
    cd ../..
    rm -rf kcov-33

    mkdir -p target/cov/aluminum
    mkdir -p target/cov/lib
    mkdir -p target/cov/merged
    tmp/usr/local/bin/kcov --exclude-pattern=/.cargo,/usr/lib,tests/ --verify target/cov/aluminum target/debug/aluminum-*
    tmp/usr/local/bin/kcov --exclude-pattern=/.cargo,/usr/lib,tests/ --verify target/cov/lib target/debug/lib-*
    tmp/usr/local/bin/kcov --merge target/cov/merged target/cov/aluminum target/cov/lib

    bash <(curl -s https://codecov.io/bash) -s target/cov/merged
fi
