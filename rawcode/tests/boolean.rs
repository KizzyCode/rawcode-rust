use rawcode::{error::Error, RawcodeConstSize};

/// Tests encoding of valid value
#[test]
fn encode_valid() -> Result<(), Error> {
    let tests = [(true, b"\xff"), (false, b"\x00")];

    for (value, expected) in tests {
        let mut encoded = [0; bool::SIZE];
        rawcode::to_slice(&value, &mut encoded)?;
        assert_eq!(&encoded, expected);
    }
    Ok(())
}

/// Tests decoding of valid values
#[test]
fn decode_valid() -> Result<(), Error> {
    let tests = [(b"\xff", true), (b"\x00", false)];

    for (encoded, expected) in tests {
        let decoded: bool = rawcode::from_slice(encoded)?;
        assert_eq!(decoded, expected);
    }
    Ok(())
}

/// Tests decoding of invalid values
#[test]
fn decode_invalid() -> Result<(), Error> {
    let tests = [b"".as_slice(), b"\x01".as_slice(), b"\xFF\xFF".as_slice()];

    for invalid in tests {
        let result: Result<bool, Error> = rawcode::from_slice(invalid);
        result.expect_err("Unexpected success");
    }
    Ok(())
}
