#!/bin/bash

# invoke the given command with any environment variables whose name starts with `DIOM` unset

while IFS='=' read -r -d '' n _v; do
    if [[ "$n" = DIOM* ]]; then
        unset -v "$n"
    fi
done < <(env -0)

exec "$@"
