if [[ $TRAVIS_RUST_VERSION = 'stable' ]]; then
    # FIXME: Dirty hack to get kcov working for now...
    wget https://github.com/SimonKagstrom/kcov/archive/1e13fa40156c6717d1efd1f1c13fee8a94b8eea0.zip
    unzip 1e13fa40156c6717d1efd1f1c13fee8a94b8eea0.zip
    mv kcov-1e13fa40156c6717d1efd1f1c13fee8a94b8eea0 kcov-master
    cd kcov-master
    mkdir build
    cd build
    cmake ..
    make
    make install DESTDIR=../tmp
    cd ../..
    for file in target/debug/aluminum-*; do
        ./kcov-master/tmp/usr/local/bin/kcov --exclude-pattern=/.cargo,target/ --verify target/kcov target/debug/aluminum-*
    done

    ./kcov-master/tmp/usr/local/bin/kcov --coveralls-id=$TRAVIS_JOB_ID --exclude-pattern=/.cargo,target/ --verify target/kcov target/debug/lib-*
fi
