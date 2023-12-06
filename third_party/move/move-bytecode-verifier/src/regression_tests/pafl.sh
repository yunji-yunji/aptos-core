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
        -ryj)
            # Clear env variable
            # add if existing
            rm /home/y23kim/rust/last_rust/aptos-core/target/miri/x86_64-unknown-linux-gnu/debug/deps/move_bytecode_verifier-*
            PAFL=${TARGET} PAFL_TARGET_PREFIX=${PREFIX} \
            cargo +fuzz_yj miri test \
                regression_tests::fuzz::miri_path_fuzz -- \
                --exact
            ;;
        -ryjvm)
            # move_vm_types
            # cargo +fuzz_yj miri test --package move-vm-integration-tests --lib -- tests::bad_storage_tests::test_malformed_resource --exact --nocapture
            PAFL=${TARGET} PAFL_TARGET_PREFIX=${PREFIX} \
            cargo +fuzz_yj miri test \
                values::value_tests::* -- \
                --exact
            ;;
            # cargo +fuzz_yj miri test tests::bad_storage_tests::test_malformed_resource -- --exact
            # MIRIFLAGS=-Zmiri-disable-isolation cargo +fuzz_yj miri test --package move-core-types --lib -- unit_tests::identifier_test

        *)
            echo "invalid argument $1"
            exit 1
            ;;
    esac
    shift
done
