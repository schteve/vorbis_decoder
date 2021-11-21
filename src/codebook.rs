use crate::{huffman::HuffmanTree, util};
use bitstream_io::{BitRead, BitReader};
use thiserror::Error;

#[derive(Debug, Default, PartialEq)]
pub struct Codebook {
    dimensions: u16,
    entries: u32,
    ordered: bool,
    sparse: Option<bool>,
    codeword_lengths: Vec<Option<u8>>,
    lookup_type: u8,
    vector_lookup_table: Option<VectorLookupTable>,
    huffman_tree: HuffmanTree,
}

impl Codebook {
    pub fn decode<R, E>(reader: &mut BitReader<R, E>) -> Result<Self, CodebookError>
    where
        R: std::io::Read,
        E: bitstream_io::Endianness,
    {
        let sync_pattern: [u8; 3] = [reader.read(8)?, reader.read(8)?, reader.read(8)?];
        if sync_pattern != [0x42, 0x43, 0x56] {
            return Err(CodebookError::InvalidSyncPattern(sync_pattern));
        }

        let dimensions = reader.read(16)?;
        let entries: u32 = reader.read(24)?;

        let ordered = reader.read_bit()?;

        let mut sparse = None;
        let mut codeword_lengths: Vec<Option<u8>> = Vec::new();
        if ordered == false {
            // The codeword list is not length ordered and we need to read each codeword length one-by-one
            sparse = Some(reader.read_bit()?);
            for _ in 0..entries {
                if sparse == Some(true) {
                    let flag: bool = reader.read_bit()?;
                    if flag == true {
                        let length = reader.read::<u8>(5)? + 1;
                        codeword_lengths.push(Some(length));
                    } else {
                        // This entry is unused. Mark it as such.
                        codeword_lengths.push(None);
                    }
                } else {
                    let length = reader.read::<u8>(5)? + 1;
                    codeword_lengths.push(Some(length));
                }
            }
        } else {
            // The codeword list is encoded in ascending length order. Rather than reading a length for every
            // codeword, we read the number of codewords per length.
            let mut current_entry: u32 = 0;
            let mut current_length = reader.read::<u8>(1)? + 1;
            while current_entry < entries {
                let bits_to_read = util::ilog((entries - current_entry) as i32);
                let number = reader.read::<u32>(bits_to_read)?;
                for _ in 0..number {
                    codeword_lengths.push(Some(current_length));
                }
                current_entry += number;
                current_length += 1;
            }
            if current_entry > entries {
                return Err(CodebookError::TooManyEntries(current_entry));
            }
        }

        // Read vector lookup table
        let lookup_type: u8 = reader.read(4)?;
        let vector_lookup_table = match lookup_type {
            0 => None,
            1 | 2 => {
                let minimum_value = util::float32_unpack(reader.read(32)?);
                let delta_value = util::float32_unpack(reader.read(32)?);
                let value_bits = reader.read::<u8>(4)? + 1;
                let sequence_p: bool = reader.read_bit()?;
                let lookup_values = if lookup_type == 1 {
                    util::lookup1_values(entries, dimensions as u32)
                } else {
                    entries * dimensions as u32
                };
                let multiplicands: Vec<u32> = (0..lookup_values)
                    .map(|_| reader.read(value_bits as u32))
                    .collect::<Result<_, _>>()?;

                Some(VectorLookupTable {
                    minimum_value,
                    delta_value,
                    value_bits,
                    sequence_p,
                    lookup_values,
                    multiplicands,
                })
            }
            x => return Err(CodebookError::InvalidLookupType(x)),
        };

        // Set up Huffman tree
        let mut huffman_tree = HuffmanTree::new();
        for (value, length) in codeword_lengths.iter().enumerate() {
            if let Some(len) = length {
                huffman_tree.add_node(*len, value as u32);
            }
        }

        Ok(Self {
            dimensions,
            entries,
            ordered,
            sparse,
            codeword_lengths,
            lookup_type,
            vector_lookup_table,
            huffman_tree,
        })
    }
}

#[derive(Debug, PartialEq)]
pub struct VectorLookupTable {
    minimum_value: f32,
    delta_value: f32,
    value_bits: u8,
    sequence_p: bool,
    lookup_values: u32,
    multiplicands: Vec<u32>,
}

#[derive(Debug, Error)]
pub enum CodebookError {
    #[error("Invalid sync pattern: {0:?}")]
    InvalidSyncPattern([u8; 3]),

    #[error("Too many entries: {0}")]
    TooManyEntries(u32),

    #[error("Invalid lookup type: {0}")]
    InvalidLookupType(u8),

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

        // Frampton codebook 0
        let input = [66, 67, 86, 1, 0, 8, 0, 0, 0, 49, 76, 32, 197, 128];
        let mut cursor = Cursor::new(input);
        let mut reader = BitReader::endian(&mut cursor, LittleEndian);
        let mut codebook = Codebook::decode(&mut reader).unwrap();
        codebook.huffman_tree = HuffmanTree::new(); // This is generated purely from codeword_lengths so don't bother testing it
        assert_eq!(
            codebook,
            Codebook {
                dimensions: 1,
                entries: 8,
                ordered: false,
                sparse: Some(false),
                codeword_lengths: vec![
                    Some(1),
                    Some(3),
                    Some(4),
                    Some(7),
                    Some(2),
                    Some(5),
                    Some(6),
                    Some(7)
                ],
                lookup_type: 0,
                vector_lookup_table: None,
                huffman_tree: HuffmanTree::new(),
            }
        );

        // Invalid sync pattern
        let input = [1, 2, 3];
        let mut cursor = Cursor::new(input);
        let mut reader = BitReader::endian(&mut cursor, LittleEndian);
        let err = Codebook::decode(&mut reader).unwrap_err();
        assert!(matches!(err, CodebookError::InvalidSyncPattern([1, 2, 3])));

        // Too many entries
        let input = [66, 67, 86, 1, 0, 8, 0, 0, 61];
        let mut cursor = Cursor::new(input);
        let mut reader = BitReader::endian(&mut cursor, LittleEndian);
        let err = Codebook::decode(&mut reader).unwrap_err();
        assert!(matches!(err, CodebookError::TooManyEntries(15)));

        // Invalid lookup type
        let input = [66, 67, 86, 1, 0, 8, 0, 0, 0, 49, 76, 32, 197, 188]; // Change lookup_type to 0b1111
        let mut cursor = Cursor::new(input);
        let mut reader = BitReader::endian(&mut cursor, LittleEndian);
        let err = Codebook::decode(&mut reader).unwrap_err();
        assert!(matches!(err, CodebookError::InvalidLookupType(15)));

        // IOError
        let input = [];
        let mut cursor = Cursor::new(input);
        let mut reader = BitReader::endian(&mut cursor, LittleEndian);
        let err = Codebook::decode(&mut reader).unwrap_err();
        match err {
            CodebookError::IOError(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => (),
            x => panic!("Unexpected result: {:?}", x),
        }
    }
}
