#!/usr/bin/env bash
# Helper for regenerating FlipperZero SDK bindings using Docker builder.

set -e -o pipefail

cd "$(dirname "${BASH_SOURCE[0]}")"

if [[ "$#" -ne 1 ]]; then
    printf >&2 'Usage: %s BRANCH\n' "${0}"
    exit 2
fi

BRANCH="${1}"

function cleanup {
    if test -n "${CONTAINER}"; then
        docker container rm "${CONTAINER}" >/dev/null
    fi
}

trap cleanup EXIT

printf >&2 'Generating bindings for flipperzero-firmware@%s\n' "${BRANCH}"
IMAGE=$(DOCKER_BUILDKIT=1 docker build --file Dockerfile --quiet --build-arg BRANCH="${BRANCH}" ..)

CONTAINER=$(docker container create --read-only "${IMAGE}")
docker container cp "${CONTAINER}":bindings.rs ../crates/sys/src/bindings.rs
