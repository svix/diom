#!/usr/bin/env bash

cargo llvm-cov nextest --profile ci --no-clean
status=$?

if [[ $status -eq 0 ]]; then
    cargo llvm-cov report --html
    mv target/llvm-cov/html ./coverage-html
    coverage_percentage=$(cargo llvm-cov report --json | jq '.data[].totals.lines.percent')
    if [[ -n "$GITHUB_OUTPUT" ]]; then
        printf 'coverage_percent=%.2f\n' "$coverage_percentage" >>"$GITHUB_OUTPUT"
    fi
fi

mv target/nextest/ci/junit.xml ./junit.xml
exit $status
