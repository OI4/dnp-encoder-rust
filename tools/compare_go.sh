#!/usr/bin/env bash
set -euo pipefail
# compare_go.sh
# Compares Rust encoder output with Go reference implementation.
# Requirements:
#   - go installed
#   - git (if automatic clone is needed)
#   - Rust binary (cargo build) available
# Behavior:
#   If GO_DNP_REPO is unset, the script clones the development branch of the
#   official repository (depth=1) into a temporary directory.
# Usage:
#   ./tools/compare_go.sh < testcases.txt
# Each line is raw input for both encoders.

DEFAULT_GO_GIT_URL="https://github.com/OI4/dnp-encoder-go.git"  # 'tree/development' web URL is not a clone URL.
GO_BRANCH="development"

CLEANUP_CLONE=0
if [[ -z "${GO_DNP_REPO:-}" ]]; then
  if ! command -v git >/dev/null; then
    echo "git required for automatic clone fallback" >&2
    exit 2
  fi
  echo "[info] GO_DNP_REPO not set – cloning ${DEFAULT_GO_GIT_URL} (branch ${GO_BRANCH})" >&2
  TMP_DIR="$(mktemp -d)"
  git clone --depth 1 -b "${GO_BRANCH}" "${DEFAULT_GO_GIT_URL}" "${TMP_DIR}" >&2
  GO_DNP_REPO="${TMP_DIR}"
  CLEANUP_CLONE=1
fi

if ! command -v go >/dev/null; then
  echo "go toolchain not found" >&2
  exit 3
fi

# Build Go reference (assumes a main producing encoder; adjust path if needed)
pushd "${GO_DNP_REPO}" >/dev/null
GO_BIN="$(mktemp)"
go build -o "${GO_BIN}" ./... >&2 || { echo "go build failed" >&2; exit 4; }
popd >/dev/null

# Build Rust CLI
cargo build --quiet --features std,alloc
RUST_BIN=target/debug/oi4-dnp-encoding-cli

fail=0
while IFS= read -r line; do
  rust_out=$(echo -n "${line}" | "${RUST_BIN}" encode)
  go_out=$(echo -n "${line}" | "${GO_BIN}" encode 2>/dev/null || true)
  if [[ "${rust_out}" != "${go_out}" ]]; then
    echo "DIFF: input='${line}' rust='${rust_out}' go='${go_out}'" >&2
    fail=1
  fi
done

rm -f "${GO_BIN}"
if [[ ${CLEANUP_CLONE} -eq 1 ]]; then
  rm -rf "${GO_DNP_REPO}"
fi
exit ${fail}
