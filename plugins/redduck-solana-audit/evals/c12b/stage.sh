#!/usr/bin/env bash
# Copies this eval case's program into the run's empty workspace as ./program.
# Runs only when `claude plugin eval` is given --scaffold, and only from a
# repository checkout (the fixtures live in benchmark/, which is not shipped).
set -euo pipefail
CASE_ID="c12b"
dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
while [ ! -d "$dir/benchmark/fixtures/$CASE_ID" ]; do
  parent="$(dirname "$dir")"
  if [ "$parent" = "$dir" ]; then
    echo "fixture $CASE_ID not found: run evals from a repository checkout" >&2
    exit 1
  fi
  dir="$parent"
done
mkdir -p program
cp -R "$dir/benchmark/fixtures/$CASE_ID/program/." program/
