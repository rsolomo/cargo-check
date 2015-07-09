# cargo-check

[![Build Status](https://travis-ci.org/rsolomo/cargo-check.svg?branch=master)](https://travis-ci.org/rsolomo/cargo-check)

This is a wrapper around `cargo rustc -- -Zno-trans`.
It can be helpful for running a faster compile if you
only need correctness checks.

## Installation

- `cargo build --release`
- add `target/release/cargo-check` into your `$PATH`

Note that additonal `cargo rustc` options can still be passed through.

In other words, this:

```
cargo check -v -- -Zprint-link-args
```

Should do the same thing as this:

```
cargo rustc -v -- -Zno-trans -Zprint-link-args
```
