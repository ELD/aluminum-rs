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

    for file in target/debug/aluminum-*; do
        mkdir -p "target/cov/$(basename $file)"
        tmp/usr/local/bin/kcov --exclude-pattern=/.cargo,/usr/lib,tests/ --verify "target/cov/$(basename $file)" "$file"
    done

    tmp/usr/local/bin/kcov --exclude-pattern=/.cargo,/usr/lib,tests/ --verify target/cov/lib target/debug/lib-*

    bash <(curl -s https://codecov.io/bash)
fi
