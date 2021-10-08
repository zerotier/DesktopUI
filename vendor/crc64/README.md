# CRC64

[![Build Status](https://travis-ci.org/badboy/crc64-rs.svg?branch=master)](https://travis-ci.org/badboy/crc64-rs)
[![crates.io](http://meritbadge.herokuapp.com/crc64)](https://crates.io/crates/crc64)

A 5-line\* CRC64 implementation in Rust.

\*: Yes, I cheated *a bit*. It is [antirez](https://github.com/antirez)' implementation of the [CRC64 algorithm for Redis][crc64.c], which basically consists of one huge table. See [lib.rs](src/lib.rs) for the exact constants used. Oh, since v0.2.0 I cheated even more. It's not 5 lines anymore, more like 25.

## Build

```
cargo build --release
```

## Usage

As a library:

```rust
use crc64::crc64;
crc64::crc64(0, "123456789".as_bytes());
```

As a standalone application:

```
$ ./target/release/crc64 src/crc64/lib.rs
```

## Tests

Run tests with:

```
cargo test
```

## Contribute

If you find bugs or want to help otherwise, please [open an issue](https://github.com/badboy/crc64-rs/issues).  

## License

BSD. See [LICENSE](LICENSE).  
Redis and the code I used is also released under a BSD license. See [crc64.c][].

[crc64.c]: https://github.com/antirez/redis/blob/unstable/src/crc64.c
