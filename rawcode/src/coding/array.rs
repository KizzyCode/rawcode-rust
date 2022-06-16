//! Implements the encoding of arrays

use crate::{
    coding::{RawcodeConstSize, RawcodeDecode, RawcodeEncode},
    error::Error,
};

// Implements rawcode for arrays
impl<const LEN: usize, T> RawcodeConstSize for [T; LEN]
where
    T: RawcodeConstSize,
{
    const SIZE: usize = T::SIZE * LEN;
}
impl<const LEN: usize, T> RawcodeDecode for [T; LEN]
where
    T: RawcodeDecode,
{
    fn decode(buf: &[u8]) -> Result<Self, Error> {
        // Validate the input length
        match buf.len() {
            len if len > Self::SIZE => return Err(e!("Encoded data is too long")),
            len if len < Self::SIZE => return Err(e!("Encoded data is too short")),
            _ => (/* all ok */),
        }

        // Parse all elements
        let mut elements: [Option<T>; LEN] = [(); LEN].map(|_| None);
        for index in 0..LEN {
            let buf = &buf[index * T::SIZE..][..T::SIZE];
            let element = T::decode(buf)?;
            elements[index] = Some(element);
        }

        // Create array
        let elements = elements.map(|element| element.expect("Array element is not initialized?!"));
        Ok(elements)
    }
}
impl<const LEN: usize, T> RawcodeEncode for [T; LEN]
where
    T: RawcodeEncode,
{
    fn encode(&self, buf: &mut [u8]) -> Result<(), Error> {
        // Validate the buffer length
        match buf.len() {
            len if len > Self::SIZE => return Err(e!("Target buffer is too large")),
            len if len < Self::SIZE => return Err(e!("Target buffer is too small")),
            _ => (/* all ok */),
        }

        // Encode all elements
        for (index, element) in self.iter().enumerate() {
            let buf = &mut buf[index * T::SIZE..][..T::SIZE];
            element.encode(buf)?;
        }
        Ok(())
    }
}
