use crate::{huffman::HuffmanTree, util};
use bitstream_io::{BitRead, BitReader};
use std::cmp::Ordering;

#[derive(Debug, Default)]
pub struct Codebook {
    dimensions: u16,
    entries: u32,
    ordered: bool,
    sparse: bool,
    codeword_lengths: Vec<Option<u8>>,
    lookup_type: u8,
    vector_lookup_table: Option<VectorLookupTable>,
    huffman_tree: HuffmanTree,
}

impl Codebook {
    pub fn decode<R, E>(reader: &mut BitReader<R, E>) -> Self
    where
        R: std::io::Read,
        E: bitstream_io::Endianness,
    {
        let a: u8 = reader.read(8).unwrap();
        let b: u8 = reader.read(8).unwrap();
        let c: u8 = reader.read(8).unwrap();
        let sync_pattern = [a, b, c];
        assert_eq!(sync_pattern, [0x42, 0x43, 0x56]);

        let mut codebook: Self = Default::default();

        codebook.dimensions = reader.read(16).unwrap();
        codebook.entries = reader.read(24).unwrap();

        codebook.ordered = reader.read::<u8>(1).unwrap() == 1;

        if codebook.ordered == false {
            // The codeword list is not length ordered and we need to read each codeword length one-by-one
            codebook.sparse = reader.read::<u8>(1).unwrap() == 1;
            for _ in 0..codebook.entries {
                if codebook.sparse == true {
                    let flag: bool = reader.read::<u8>(1).unwrap() == 1;
                    if flag == true {
                        let length = reader.read::<u8>(1).unwrap() + 1;
                        codebook.codeword_lengths.push(Some(length));
                    } else {
                        // This entry is unused. Mark it as such.
                        codebook.codeword_lengths.push(None);
                    }
                } else {
                    let length = reader.read::<u8>(1).unwrap() + 1;
                    codebook.codeword_lengths.push(Some(length));
                }
            }
        } else {
            // The codeword list is encoded in ascending length order. Rather than reading a length for every
            // codeword, we read the number of codewords per length.
            let mut current_entry = 0;
            let mut current_length = reader.read::<u8>(1).unwrap() + 1;
            while current_entry < codebook.entries {
                let bits_to_read = util::ilog(codebook.entries - current_entry);
                let number = reader.read::<u32>(bits_to_read).unwrap();
                for _ in 0..number {
                    codebook.codeword_lengths.push(Some(current_length));
                }
                current_entry += number;
                current_length += 1;

                match current_entry.cmp(&codebook.entries) {
                    Ordering::Less => (),
                    Ordering::Equal => break,
                    Ordering::Greater => panic!("Error: too many codebook entries!"),
                }
            }
            assert_eq!(current_entry, codebook.entries);
        }

        // Read vector lookup table
        let lookup_type: u8 = reader.read(4).unwrap();
        codebook.vector_lookup_table = match lookup_type {
            0 => None,
            1 | 2 => {
                let minimum_value = util::float32_unpack(reader.read(32).unwrap());
                let delta_value = util::float32_unpack(reader.read(32).unwrap());
                let value_bits = reader.read::<u8>(4).unwrap() + 1;
                let sequence_p: bool = reader.read::<u8>(1).unwrap() == 1;
                let lookup_values = if lookup_type == 1 {
                    util::lookup1_values(codebook.entries, codebook.dimensions as u32)
                } else {
                    codebook.entries * codebook.dimensions as u32
                };
                let multiplicands: Vec<u32> = (0..lookup_values)
                    .map(|_| reader.read(value_bits as u32).unwrap())
                    .collect();

                Some(VectorLookupTable {
                    minimum_value,
                    delta_value,
                    value_bits,
                    sequence_p,
                    lookup_values,
                    multiplicands,
                })
            }
            x => panic!("Reserved codebook lookup type: {}", x),
        };

        // Set up Huffman tree
        for (value, length) in codebook.codeword_lengths.iter().enumerate() {
            if let Some(len) = length {
                codebook.huffman_tree.add_node(*len, value as u32);
            }
        }

        codebook
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_() {}
}
