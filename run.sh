#!/bin/bash

set -e

IMAGE=niax/aoc2023:1

docker image inspect "${IMAGE}" >/dev/null 2>&1 || docker build .docker -t "${IMAGE}"

exec docker run --rm -i -v "${PWD}:/code" -v "${PWD}/.cargo:/usr/local/cargo/registry" "${IMAGE}" "${@}"
