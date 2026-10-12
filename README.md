# ormus-tools

Rust versions of the Ormus command line tools.

This repo replaces the TypeScript tooling one piece at a time. The source idea is tracked in Linear as ORM-339: fewer layers between us and the machine, and more time spent on checks that let agents verify their own work.

## Crates

- `ormus-core`: shared config, auth, and Linear and GitHub clients
- `ormus-cli`: the `ormus` binary, with the same commands as the TypeScript version in ormus-agent-harness
- `merge-clerk`: the Liquid Gold merge clerk (placeholder)
- `review-gate`: the review gate and its validation harness (placeholder)

Commands that have not been ported yet print `not yet ported` and exit with code 2.

## Build

    cargo build
    cargo test

CI (to be moved to .github/workflows/ci.yml from ci/github-ci.yml) runs `cargo fmt --check`, `cargo clippy -D warnings`, and `cargo test`.
