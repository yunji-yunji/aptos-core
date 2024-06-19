#!/bin/bash

CURDIR=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &> /dev/null && pwd)
TARGET=${CURDIR}/fuzz
PREFIX=third_party/move
RUNTIMEPATH="${CURDIR}/traces/000.trace3.json"
ROOTDIR=${CURDIR}/../../../../..

echo $CURDIR
echo $TARGET
echo $RUNTIMEPATH
echo $ROOTDIR

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
            rm -rf ${TARGET}
            # rm -r ./target/miri
            # rm ../../../../aptos-core/target/miri/x86_64-unknown-linux-gnu/debug/deps/move_bytecode_verifier-*

            cargo +fuzz test \
                regression_tests::fuzz::miri_path_fuzz -- \
                --exact
            ;;
        -staticdump)
	    echo "static dump!"
	    # clean up
            cargo +fuzz clean
            rm -rf ${TARGET}
	    rm ${ROOTDIR}/target/miri/x86_64-unknown-linux-gnu/debug/deps/move_bytecode_verifier-*
	    
	    RUSTFLAGS="-A warnings" \
            DUMP_CFG_JSON=${TARGET} \
	    PAFL_TARGET_PREFIX=${PREFIX} \
            cargo +fuzz miri test \
                regression_tests::fuzz::miri_path_fuzz -- \
                --exact
            ;;
        -runtimedump)
            # clear env var
	    rm ${ROOTDIR}/target/miri/x86_64-unknown-linux-gnu/debug/deps/move_bytecode_verifier-*
            
	    # DUMP_TRACE="/home/y23kim/rust/last_rust/aptos-core/third_party/move/move-bytecode-verifier/src/regression_tests/traces/000.trace3.json" \
	    RUSTFLAGS="-A warnings" \
	    DUMP_TRACE=${RUNTIMEPATH} \
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
