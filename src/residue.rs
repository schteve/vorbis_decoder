use bitstream_io::{BitRead, BitReader};

#[derive(Debug)]
pub struct Residue {
    vorbis_residue_type: u16,
    begin: u32,
    end: u32,
    partition_size: u32,
    classifications: u8,
    classbook: u8,
    cascade: Vec<u8>,
    books: Vec<Vec<Option<u8>>>,
}

impl Residue {
    pub fn decode<R, E>(reader: &mut BitReader<R, E>) -> Self
    where
        R: std::io::Read,
        E: bitstream_io::Endianness,
    {
        let vorbis_residue_type = reader.read::<u16>(16).unwrap();
        if matches!(vorbis_residue_type, 0..=2) == false {
            panic!("Invalid residue type {}", vorbis_residue_type);
        }

        let begin = reader.read(24).unwrap();
        let end = reader.read(24).unwrap();
        let partition_size = reader.read::<u32>(24).unwrap() + 1;
        let classifications = reader.read::<u8>(6).unwrap() + 1;
        let classbook = reader.read(8).unwrap();

        let cascade: Vec<u8> = (0..classifications)
            .map(|_| {
                let low_bits = reader.read::<u8>(3).unwrap();
                let bitflag: bool = reader.read::<u8>(1).unwrap() == 1;
                let high_bits = if bitflag == true {
                    reader.read::<u8>(5).unwrap()
                } else {
                    0
                };
                high_bits * 8 + low_bits
            })
            .collect();

        let books: Vec<Vec<Option<u8>>> = cascade
            .iter()
            .map(|cascade_elem| {
                (0..8)
                    .map(|j| {
                        if cascade_elem & (1 << j) != 0 {
                            Some(reader.read(8).unwrap())
                        } else {
                            None
                        }
                    })
                    .collect()
            })
            .collect();

        // TODO: validate:
        // Any codebook number greater than the maximum numbered codebook set up in this stream also renders the stream undecodable.
        // All codebooks in array [residue_books] are required to have a value mapping.
        // The presence of codebook in array [residue_books] without a value mapping (maptype equals zero) renders the stream undecodable.

        Self {
            vorbis_residue_type,
            begin,
            end,
            partition_size,
            classifications,
            classbook,
            cascade,
            books,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_() {}
}
