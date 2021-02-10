#!/usr/bin/env bash
SCRIPTDIR=$PWD

# test each pallets tests
for d in $(ls -d ./pallets/*/) ; do
    cd "$SCRIPTDIR/$d" && WASM_BUILD_TOOLCHAIN=nightly-2020-10-05 cargo test
done

# test wasm utils
cd "$SCRIPTDIR/wasm-utils" && ./test.sh

# test gadget tests
cd "$SCRIPTDIR/gadgets" && WASM_BUILD_TOOLCHAIN=nightly-2020-10-05 cargo test