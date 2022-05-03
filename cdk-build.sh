#!/bin/bash
LAMBDA_DIR=$(pwd)/cdk_lambda_rust
LAMBDA_ARTIFACTS="${LAMBDA_DIR}/artifacts"
[[ ! -d "${LAMBDA_ARTIFACTS}" ]] && mkdir "${LAMBDA_ARTIFACTS}"

cd cdk_lambda_rust/runtime || exit $?
cargo build --release --target x86_64-unknown-linux-musl
cd target/x86_64-unknown-linux-musl/release || exit $?
mv bootstrap "${LAMBDA_ARTIFACTS}"
