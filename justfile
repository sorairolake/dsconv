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
    csplit -s README.adoc '/^\.\.\.\.$/' '{1}'
    sed -i -n 1p xx01
    cargo -q run -- -h >> xx01
    cat xx0[0-2] > README.adoc
    rm xx0[0-2]
    echo {{ if `git status --porcelain README.adoc` == '' { 'README is up-to-date' } else { 'README has been updated!' } }}

# Generate GFM version README for crates.io
@generate-gfm-readme:
    sed -i 's/^=/==/g' README.adoc
    asciidoctor -b docbook5 -o - README.adoc | pandoc -f docbook -t gfm -o README-crates.io.md
    echo {{ if `git status --porcelain README-crates.io.md` == '' { 'README for crates.io is up-to-date' } else { 'README for crates.io has been updated!' } }}
    git restore README.adoc
