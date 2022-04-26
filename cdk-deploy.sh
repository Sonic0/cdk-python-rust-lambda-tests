#!/bin/bash
LAMBDA_DIR=$(pwd)/cdk_lambda_rust
cd cdk_lambda_rust/runtime || exit $?
cargo build --release --target x86_64-unknown-linux-musl
cd target/x86_64-unknown-linux-musl/release || exit $?
zip -rm lambda.zip bootstrap
mv lambda.zip "${LAMBDA_DIR}"
