[![License BSD-2-Clause](https://img.shields.io/badge/License-BSD--2--Clause-blue.svg)](https://opensource.org/licenses/BSD-2-Clause)
[![License MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![AppVeyor CI](https://ci.appveyor.com/api/projects/status/github/KizzyCode/rawcode-rust?svg=true)](https://ci.appveyor.com/project/KizzyCode/rawcode-rust)
[![docs.rs](https://docs.rs/rawcode/badge.svg)](https://docs.rs/rawcode)
[![crates.io](https://img.shields.io/crates/v/rawcode.svg)](https://crates.io/crates/rawcode)
[![Download numbers](https://img.shields.io/crates/d/rawcode.svg)](https://crates.io/crates/rawcode)
[![dependency status](https://deps.rs/crate/rawcode/0.2.0/status.svg)](https://deps.rs/crate/rawcode/0.2.0)


# `rawcode`
Welcome to `rawcode` 🎉

`rawcode` is a no-std-compatible, simple as-is coding. The idea is similar to
[`bincode`](https://crates.io/crates/bincode), but the format is even more primitive: No variable length coding, no
references – just a few fixed-length types: bytes, booleans, integers, (nested) arrays/lists and `StrArray`.


## Types
There's built-in support for:
- `u8`: Bytes are encoded as-is (i.e. 8 bit, network bit order)
- `bool`: Booleans are encoded as `u8` where `true => 0xFF` and `false => 0x00`
- `i8`, `u16`, `i16`, `u32`, `i32`, `u64`, `i64`, `u128`, `i128`: Integers are encoded as two's-complement in
  **little-endian** representation and always use their full width (i.e. `u16` = 2 bytes, `i128` = 16 bytes)
- `struct`s and `array`s: Fields are concatenated and encoded in order of declaration and without any padding inbetween
- `StrArray<LEN>`: This is a special wrapper around `[u8; LEN]` which ensures that it's contents are always valid UTF-8

However please note that you can also easily implement the basic traits `RawcodeConstSize` + `RawcodeEncode` +
`RawcodeDecode` to provide encoding and derivation for your own types/wrappers.


## Example
```rust ignore
use rawcode::{Rawcode, RawcodeConstSize, RawcodeDecode, RawcodeEncode, StrArray};

/// A named test struct
#[derive(Debug, PartialEq, Eq, Rawcode)]
struct Named {
    boolean: bool,
    i128_: i128,
    list: [u64; 7],
    strarray: StrArray<9>,
}

/// An unnamed test struct
#[derive(Debug, PartialEq, Eq, Rawcode)]
struct Unnamed(bool, i128, [u64; 7], StrArray<9>);


// Create test struct and target buffer
let raw = Named {
    boolean: true,
    i128_: -170141183460469231731687303715884105728,
    list: [0, 1, 2, 3, 4, 5, 6],
    strarray: StrArray::new(b"Testolope"),
};

// Encode the named struct
let mut buf = [0; Named::SIZE];
raw.encode(&mut buf).expect("Failed to encode struct");
let decoded = Unnamed::decode(&buf).expect("Failed to decode struct");

// Validate the decoding
assert_eq!(raw.boolean, decoded.0);
assert_eq!(raw.i128_, decoded.1);
assert_eq!(raw.list, decoded.2);
assert_eq!(raw.strarray, decoded.3);
```
