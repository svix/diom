#!/bin/bash

set -euo pipefail

HERE="$(readlink -f "$(dirname "$0")")"

cd "$HERE"

FIRST=0
SECOND=0
THIRD=0

on_exit() {
    set +e
    status=$?
    if [[ $FIRST != 0 ]]; then
        kill $FIRST
    fi
    if [[ $SECOND != 0 ]]; then
        kill $SECOND
    fi
    if [[ $THIRD != 0 ]]; then
        kill $THIRD
    fi
    exit $status
}

trap on_exit INT EXIT TERM QUIT

export COYOTE_ADMIN_TOKEN=admin_abcdefghijlmnopqrstuvwxyz012345

TAG=""
OFF=""

if [[ -t 1 ]]; then
    TAG="$(printf "\e[36;40m")"
    OFF="$(printf "\e[0m")"
fi

./prefix-output.sh "[${TAG}first${OFF}] " cargo run -- --config-path ./first.toml server &
FIRST=$!

sleep 10

./prefix-output.sh "[${TAG}second${OFF}]" cargo run -- --config-path ./second.toml server &
SECOND=$!

sleep 5

./prefix-output.sh "[${TAG}third${OFF}] " cargo run -- --config-path ./third.toml server &
THIRD=$!

wait
