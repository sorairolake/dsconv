#
# SPDX-License-Identifier: Apache-2.0
#
# Copyright (C) 2021 Shun Sakai
#

alias all := default

# Run default recipe
default: build

# Build a package
@build:
    cargo build

# Check a package
@check:
    cargo check

# Run tests
@test:
    cargo test

# Run the code formatter
@fmt:
    cargo fmt

# Run the linter
@clippy:
    cargo clippy -- -D warnings

# Run the linter for GitHub Actions workflow files
@lint-github-actions:
    actionlint

# Update README
@update-readme:
    csplit -s README.md '/^```/' '{1}'
    sed -i -n 1p xx01
    cargo -q run -- -h >> xx01
    cat xx0[0-2] > README.md
    rm xx0[0-2]
    echo {{ if `git status --porcelain README.md` == '' { 'README is up-to-date' } else { 'README has been updated!' } }}

# Run the code formatter for the README
@fmt-readme:
    npx prettier -w README.md

# Run the linter for the README
@lint-readme:
    npx markdownlint README.md
