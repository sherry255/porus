#!/usr/bin/env bash

DAYS=0

until rustup component add rustfmt && rustup component add clippy
do
    DAYS=$((DAYS+1))
    toolchain=nightly-$(date -d "-${DAYS} days" +%Y-%m-%d)
    rustup toolchain install ${toolchain}
    rustup default ${toolchain}
done
