use bitstream_io::{BitRead, BitReader};
use thiserror::Error;

#[derive(Debug)]
pub struct Residue {
    residue_type: u16,
    begin: u32,
    end: u32,
    partition_size: u32,
    classifications: u8,
    classbook: u8,
    cascade: Vec<u8>,
    books: Vec<Vec<Option<u8>>>,
}

impl Residue {
    pub fn decode<R, E>(reader: &mut BitReader<R, E>) -> Result<Self, ResidueError>
    where
        R: std::io::Read,
        E: bitstream_io::Endianness,
    {
        let residue_type = reader.read::<u16>(16)?;
        if matches!(residue_type, 0..=2) == false {
            return Err(ResidueError::InvalidResidueType(residue_type));
        }

        let begin = reader.read(24)?;
        let end = reader.read(24)?;
        let partition_size = reader.read::<u32>(24)? + 1;
        let classifications = reader.read::<u8>(6)? + 1;
        let classbook = reader.read(8)?;

        let cascade: Vec<u8> = (0..classifications)
            .map(|_| {
                let low_bits = reader.read::<u8>(3)?;
                let bitflag: bool = reader.read_bit()?;
                let high_bits = if bitflag == true {
                    reader.read::<u8>(5)?
                } else {
                    0
                };
                Ok(high_bits * 8 + low_bits)
            })
            .collect::<Result<_, ResidueError>>()?;

        let books: Vec<Vec<Option<u8>>> = cascade
            .iter()
            .map(|cascade_elem| {
                (0..8)
                    .map(|j| {
                        let book = if cascade_elem & (1 << j) != 0 {
                            Some(reader.read::<u8>(8)?)
                        } else {
                            None
                        };
                        Ok(book)
                    })
                    .collect::<Result<Vec<_>, ResidueError>>()
            })
            .collect::<Result<_, _>>()?;

        // TODO: validate:
        // Any codebook number greater than the maximum numbered codebook set up in this stream also renders the stream undecodable.
        // All codebooks in array [residue_books] are required to have a value mapping.
        // The presence of codebook in array [residue_books] without a value mapping (maptype equals zero) renders the stream undecodable.

        Ok(Self {
            residue_type,
            begin,
            end,
            partition_size,
            classifications,
            classbook,
            cascade,
            books,
        })
    }
}

#[derive(Debug, Error)]
pub enum ResidueError {
    #[error("Invalid residue type: {0}")]
    InvalidResidueType(u16),

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
