#!/usr/bin/env sh

set -eu

cargo_command=${RUST400_CARGO:-cargo}

run_check() {
    check_name=$1
    shift

    printf '\n==> %s\n' "$check_name"

    if "$@"; then
        printf '<== PASS: %s\n' "$check_name"
    else
        exit_code=$?
        printf '<== FAIL: %s (exit %s)\n' "$check_name" "$exit_code" >&2
        return "$exit_code"
    fi
}

run_check "Formatting" "$cargo_command" fmt --all -- --check
run_check "Clippy" "$cargo_command" clippy --all-targets --all-features -- -D warnings
run_check "Tests" "$cargo_command" test --all-targets --all-features

printf '\nAll Rust/400 quality checks passed.\n'
