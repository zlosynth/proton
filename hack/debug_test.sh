#!/usr/bin/env bash
set -euo pipefail

cd lib
TEST=$(cargo test --no-run --message-format=json | jq -r "select(.profile.test == true) | .filenames[]")
gdbgui --gdb rust-gdb ${TEST}
