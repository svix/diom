#!/usr/bin/env bash

export DIOM_LOG_LEVEL=debug
export DIOM_CLUSTER_LOG_LEVEL=debug

cargo llvm-cov nextest --profile ci --no-clean
status=$?

if [[ $status -eq 0 ]]; then
    cargo llvm-cov report --json >coverage.json
    cargo llvm-cov report --html
    mv target/llvm-cov/html ./coverage-html
fi

mv target/nextest/ci/junit.xml ./junit.xml
exit $status
