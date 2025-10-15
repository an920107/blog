#!/bin/sh

SCRIPT_DIR=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)

cd "$SCRIPT_DIR/../backend"

# Generate sqlx-data.json
cargo sqlx prepare --workspace

# Check if sqlx-data.json was modified and is not staged
if ! git diff --quiet .sqlx; then
  echo "Error: json files in .sqlx were modified by 'cargo sqlx prepare' but are not staged."
  echo "Please run 'git add backend/.sqlx' and try again."
  exit 1
fi
