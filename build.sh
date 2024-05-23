#! /usr/bin/env sh
set -euxo pipefail

(cd poseidon-impl && cargo build --release)
