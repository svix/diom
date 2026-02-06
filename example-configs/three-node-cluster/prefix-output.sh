#!/bin/bash

TAG="$1"
shift

if [[ -z "$TAG" ]]; then
    echo >&2 "Usage: tag command"
    exit 2
fi

exec > >(
    trap "" INT TERM
    sed -u "s/^/${TAG} /"
)
exec 2> >(
    trap "" INT TERM
    sed -u "s/^/${TAG} /" >&2
)

exec "$@"
