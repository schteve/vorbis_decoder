use bitstream_io::{BitRead, BitReader};

#[derive(Debug)]
pub struct Mode {
    blockflag: bool,
    window_type: u16,
    transform_type: u16,
    mapping: u8,
}

impl Mode {
    pub fn decode<R, E>(reader: &mut BitReader<R, E>) -> Self
    where
        R: std::io::Read,
        E: bitstream_io::Endianness,
    {
        let blockflag = reader.read::<u8>(1).unwrap() == 1;
        let window_type = reader.read(16).unwrap();
        assert_eq!(window_type, 0); // Zero is the only legal value in Vorbis I
        let transform_type = reader.read(16).unwrap();
        assert_eq!(transform_type, 0); // Zero is the only legal value in Vorbis I
        let mapping = reader.read(8).unwrap();
        // TODO: verify mapping is not greater than the highest number mapping in use

        Self {
            blockflag,
            window_type,
            transform_type,
            mapping,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_() {}
}
