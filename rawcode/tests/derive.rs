#![cfg(feature = "rawcode_derive")]

use rawcode::{error::Error, RawcodeConstSize, RawcodeDecode, RawcodeEncode, StrArray};
use rawcode_derive::Rawcode;

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

/// A generic test struct
#[derive(Debug, PartialEq, Eq, Rawcode)]
struct Generic<T>
where
    T: RawcodeConstSize + RawcodeEncode + RawcodeDecode,
{
    u64_: u64,
    wrapped: T,
    i8_: i8,
}

/// Tests derive for a named struct
#[test]
fn named() -> Result<(), Error> {
    // Create struct and buffer
    let raw = Named {
        boolean: true,
        i128_: 170141183460469231731687303715884105727,
        list: [0, 1, 2, 3, 4, 5, 6],
        strarray: StrArray::try_from("Testolope")?,
    };
    let mut buf = [0; Named::SIZE];

    // Perform decode-encode-cycle
    raw.encode(&mut buf)?;
    let decoded = Named::decode(&buf)?;
    assert_eq!(raw, decoded);
    Ok(())
}

/// Tests derive for a tuple struct
#[test]
fn unnamed() -> Result<(), Error> {
    // Create struct and buffer
    let raw =
        Unnamed(true, -170141183460469231731687303715884105728, [0, 1, 2, 3, 4, 5, 6], StrArray::new(b"Testolope"));
    let mut buf = [0; Unnamed::SIZE];

    // Perform decode-encode-cycle
    raw.encode(&mut buf)?;
    let decoded = Unnamed::decode(&buf)?;
    assert_eq!(raw, decoded);
    Ok(())
}

/// Tests derive for a tuple struct
#[test]
fn generic() -> Result<(), Error> {
    // Create struct and buffer
    let raw = Generic { u64_: 7, wrapped: StrArray::new(b"Testolope"), i8_: 4 };
    let mut buf = [0; Generic::<StrArray<9>>::SIZE];

    // Perform decode-encode-cycle
    raw.encode(&mut buf)?;
    let decoded = Generic::decode(&buf)?;
    assert_eq!(raw, decoded);
    Ok(())
}
