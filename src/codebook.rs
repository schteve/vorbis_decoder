use crate::{huffman::HuffmanTree, util};
use bitstream_io::{BitRead, BitReader};
use thiserror::Error;

#[derive(Debug, Default)]
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

        let ordered = reader.read::<u8>(1)? == 1;

        let mut sparse = None;
        let mut codeword_lengths: Vec<Option<u8>> = Vec::new();
        if ordered == false {
            // The codeword list is not length ordered and we need to read each codeword length one-by-one
            sparse = Some(reader.read::<u8>(1)? == 1);
            for _ in 0..entries {
                if sparse == Some(true) {
                    let flag: bool = reader.read::<u8>(1)? == 1;
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
            if current_entry != entries {
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
                let sequence_p: bool = reader.read::<u8>(1)? == 1;
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

#[derive(Debug)]
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
