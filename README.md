# cargo-check

[![Build Status](https://travis-ci.org/rsolomo/cargo-check.svg?branch=master)](https://travis-ci.org/rsolomo/cargo-check)
[![](http://meritbadge.herokuapp.com/cargo-check)](https://crates.io/crates/cargo-check)

This is a wrapper around `cargo rustc -- -Zno-trans`.
It can be helpful for running a faster compile if you
only need correctness checks.

## Installation

Install with `cargo install cargo-check`

Note that additonal `cargo rustc` options can still be passed through.

In other words, this:

```
cargo check -v -- -Zprint-link-args
```

Should do the same thing as this:

```
cargo rustc -v -- -Zprint-link-args -Zno-trans
```

And this:

```
cargo check --lib
```

Will do the same thing as this:

```
cargo rustc --lib -- -Zno-trans
```
