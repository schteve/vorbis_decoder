use bitstream_io::{BitRead, BitReader};

#[derive(Debug, Default)]
pub struct Floor1 {
    reserved: u16,
}

impl Floor1 {
    pub fn decode<R, E>(reader: &mut BitReader<R, E>) -> Self
    where
        R: std::io::Read,
        E: bitstream_io::Endianness,
    {
        let reserved = reader.read(16).unwrap();
        assert_eq!(reserved, 0);
        Self { reserved }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_() {}
}
