use rawcode::{error::Error, RawcodeConstSize};

/// Tests encoding of valid value
#[test]
fn encode_valid() -> Result<(), Error> {
    let tests = [(0u16, b"\x00\x00"), (255, b"\xff\x00"), (65_535, b"\xff\xff")];

    for (value, expected) in tests {
        let mut encoded = [0; u16::SIZE];
        rawcode::to_slice(&value, &mut encoded)?;
        assert_eq!(&encoded, expected);
    }
    Ok(())
}

/// Tests decoding of valid values
#[test]
fn decode_valid() -> Result<(), Error> {
    let tests = [(b"\x00\x00", 0), (b"\xff\x00", 255), (b"\xff\xff", 65_535)];

    for (encoded, expected) in tests {
        let decoded: u16 = rawcode::from_slice(encoded)?;
        assert_eq!(decoded, expected);
    }
    Ok(())
}

/// Tests decoding of invalid values
#[test]
fn decode_invalid() -> Result<(), Error> {
    let tests = [b"".as_slice(), b"\x01".as_slice(), b"\xFF\xFF\xFF".as_slice()];

    for invalid in tests {
        let result: Result<u16, Error> = rawcode::from_slice(invalid);
        result.expect_err("Unexpected success");
    }
    Ok(())
}
