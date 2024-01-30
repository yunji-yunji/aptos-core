#!/bin/bash

CURDIR=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &> /dev/null && pwd)
TARGET=${CURDIR}/fuzz
PREFIX=third_party/move
# echo $CURDIR
# echo $TARGET

while test $# -gt 0; do
    case "$1" in
        -b)
            # clean up
            cargo +fuzz clean
            rm -rf ${TARGET}

            # build the crates
            PAFL=${TARGET} PAFL_TARGET_PREFIX=${PREFIX} \
            cargo +fuzz test \
                regression_tests::fuzz::miri_path_fuzz -- \
                --exact
            ;;
        -r)
            # run the fuzzer
            PAFL=${TARGET} PAFL_TARGET_PREFIX=${PREFIX} \
            cargo +fuzz miri test \
                regression_tests::fuzz::miri_path_fuzz -- \
                --exact
            ;;
        -btest)
            # clean up
            cargo +fuzz clean
            rm -rf ${TARGET}

            # rm /home/y23kim/rust/last_rust/aptos-core/target/miri/x86_64-unknown-linux-gnu/debug/deps/move_bytecode_verifier-*

            # build the crates
            PAFL_EMIT=${TARGET} PAFL_TARGET_PREFIX=${PREFIX} \
            cargo +fuzz test \
                regression_tests::fuzz::miri_path_fuzz -- \
                --exact
            ;;
        *)
            echo "invalid argument $1"
            exit 1
            ;;
    esac
    shift
done
