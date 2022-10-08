# Crossbeam: support for concurrent programming

[![Build Status](https://travis-ci.org/crossbeam-rs/crossbeam.svg?branch=master)](https://travis-ci.org/crossbeam-rs/crossbeam)

Crossbeam supports concurrent programming, especially focusing on memory
management, synchronization, and non-blocking data structures.

Crossbeam consists of [several subcrates](https://github.com/crossbeam-rs).

 - `crossbeam-epoch` for treiber stacks.


# Usage

To use Crossbeam, add this to your `Cargo.toml`:

```toml
[dependencies]
crossbeam = "0.4.0"
```

For examples of what Crossbeam is capable of, see the [documentation][docs].

[docs]: https://docs.rs/crossbeam/
