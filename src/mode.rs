use bitstream_io::{BitRead, BitReader};
use thiserror::Error;

#[derive(Debug)]
pub struct Mode {
    blockflag: bool,
    window_type: u16,
    transform_type: u16,
    mapping: u8,
}

impl Mode {
    pub fn decode<R, E>(reader: &mut BitReader<R, E>) -> Result<Self, ModeError>
    where
        R: std::io::Read,
        E: bitstream_io::Endianness,
    {
        let blockflag = reader.read::<u8>(1)? == 1;
        let window_type = reader.read(16)?;
        if window_type != 0 {
            // Zero is the only legal value in Vorbis I
            return Err(ModeError::InvalidWindowType(window_type));
        }
        let transform_type = reader.read(16)?;
        if transform_type != 0 {
            // Zero is the only legal value in Vorbis I
            return Err(ModeError::InvalidTransformType(transform_type));
        }
        let mapping = reader.read(8)?;
        // TODO: verify mapping is not greater than the highest number mapping in use

        Ok(Self {
            blockflag,
            window_type,
            transform_type,
            mapping,
        })
    }
}

#[derive(Debug, Error)]
pub enum ModeError {
    #[error("Invalid window type: {0}")]
    InvalidWindowType(u16),

    #[error("Invalid transform type: {0}")]
    InvalidTransformType(u16),

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
