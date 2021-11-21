use bitstream_io::{BitRead, BitReader};
use thiserror::Error;

#[derive(Debug, Default, PartialEq)]
pub struct TimeDomainTransform {
    reserved: u16,
}

impl TimeDomainTransform {
    pub fn decode<R, E>(reader: &mut BitReader<R, E>) -> Result<Self, TimeDomainError>
    where
        R: std::io::Read,
        E: bitstream_io::Endianness,
    {
        let reserved = reader.read(16)?;
        if reserved != 0 {
            return Err(TimeDomainError::Reserved(reserved));
        }

        Ok(Self { reserved })
    }
}

#[derive(Debug, Error)]
pub enum TimeDomainError {
    #[error("Reserved value invalid: {0}")]
    Reserved(u16),

    #[error(transparent)]
    IOError(#[from] std::io::Error),
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_codebook_decode() {
        use bitstream_io::{BitReader, LittleEndian};
        use std::io::Cursor;

        // Frampton
        let input = [0, 0];
        let mut cursor = Cursor::new(input);
        let mut reader = BitReader::endian(&mut cursor, LittleEndian);
        let transform = TimeDomainTransform::decode(&mut reader).unwrap();
        assert_eq!(transform, TimeDomainTransform { reserved: 0 });

        // Invalid reserved value
        let input = [1, 2];
        let mut cursor = Cursor::new(input);
        let mut reader = BitReader::endian(&mut cursor, LittleEndian);
        let err = TimeDomainTransform::decode(&mut reader).unwrap_err();
        assert!(matches!(err, TimeDomainError::Reserved(513)));

        // IOError
        let input = [];
        let mut cursor = Cursor::new(input);
        let mut reader = BitReader::endian(&mut cursor, LittleEndian);
        let err = TimeDomainTransform::decode(&mut reader).unwrap_err();
        match err {
            TimeDomainError::IOError(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => (),
            x => panic!("Unexpected result: {:?}", x),
        }
    }
}
