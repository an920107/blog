#!/bin/sh

SCRIPT_DIR=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)

cd "$SCRIPT_DIR/../frontend"
pnpm lint
