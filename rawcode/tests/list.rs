use rawcode::{error::Error, RawcodeConstSize};

/// Tests encoding of valid value
#[test]
fn encode_valid() -> Result<(), Error> {
    let tests = [([0u8, 1, 2], b"\x00\x01\x02")];

    for (value, expected) in tests {
        let mut encoded = [0; <[u8; 3]>::SIZE];
        rawcode::to_slice(&value, &mut encoded)?;
        assert_eq!(&encoded, expected);
    }
    Ok(())
}

/// Tests decoding of valid values
#[test]
fn decode_valid() -> Result<(), Error> {
    let tests = [(b"\x00\x01\x02", [0u8, 1, 2])];

    for (encoded, expected) in tests {
        let decoded: [u8; 3] = rawcode::from_slice(encoded)?;
        assert_eq!(decoded, expected);
    }
    Ok(())
}

/// Tests decoding of invalid values
#[test]
fn decode_invalid() -> Result<(), Error> {
    let tests = [b"".as_slice(), b"\x01".as_slice(), b"\xFF\xFF\xFF\xFF".as_slice()];

    for invalid in tests {
        let result: Result<[u8; 3], Error> = rawcode::from_slice(invalid);
        result.expect_err("Unexpected success");
    }
    Ok(())
}
