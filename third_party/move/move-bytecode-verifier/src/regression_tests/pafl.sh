#!/bin/bash

CURDIR=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &> /dev/null && pwd)
TARGET=${CURDIR}/fuzz
PREFIX=third_party/move

echo $CURDIR
echo $TARGET

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
            cargo +fuzz clean
            rm -rf ${TARGET}

            # run the fuzzer
            PAFL=${TARGET} PAFL_TARGET_PREFIX=${PREFIX} 
            cargo +fuzz miri test \
                regression_tests::fuzz::miri_path_fuzz -- \
                --exact
            ;;
        -nomiri)
            ### no miri, no dump
            # clean up
            cargo +fuzz clean

            MIRIFLAGS="-Zmiri-disable-isolation" \
            cargo +fuzz test \
                regression_tests::fuzz::miri_path_fuzz -- \
                --exact
            ;;
        -staticdump)
            cargo +fuzz clean
            # rm -rf ${TARGET}
            
            RUSTFLAGS="-A warnings" \
            MIRIFLAGS="-Zmiri-disable-isolation" \
            STATIC_DUMP=${TARGET} \
            PAFL_TARGET_PREFIX=${PREFIX} \
            cargo +fuzz miri test \
                regression_tests::fuzz::miri_path_fuzz -- \
                --exact
            ;;
        -runtimedump)
            # you need to specify the TRACE_PATH 
            cargo +fuzz clean

            RUSTFLAGS="-A warnings" \
            MIRIFLAGS="-Zmiri-disable-isolation" \
            RUNTIME_DUMP=${TRACE_PATH} \
                cargo +fuzz miri test \
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
