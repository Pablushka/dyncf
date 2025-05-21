#!/bin/bash

set -o errexit
set -o nounset
set -o pipefail
set -o xtrace

readonly TARGET_HOST=pablushka@coolify.nomades.ar
readonly TARGET_ARCH=armv7-unknown-linux-gnueabihf
# readonly TARGET_ARCH=armv7-unknown-linux-gnueabihf
readonly TARGET_PATH=/home/pablushka
readonly SOURCE_PATH=./target/${TARGET_ARCH}/release/dyncf

cargo build --release --target=${TARGET_ARCH}

#rsync ${SOURCE_PATH} ${TARGET_HOST}:${TARGET_PATH}

#ssh -t ${TARGET_HOST} ${TARGET_PATH}