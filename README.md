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

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
