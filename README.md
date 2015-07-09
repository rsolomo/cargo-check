# cargo-check

This is a wrapper around `cargo rustc -- -Zno-trans`.
It can be helpful for running a faster compile if you
only need correctness checks.

## Installation

- `cargo build --release`
- add `target/release/cargo-check` into your `$PATH`

Note that additonal `cargo rustc` options can still be passed through.
