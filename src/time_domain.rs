use bitstream_io::{BitRead, BitReader};
use thiserror::Error;

#[derive(Debug, Default)]
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

    // Represents all cases of `std::io::Error`.
    #[error(transparent)]
    IOError(#[from] std::io::Error),
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_() {}
}
