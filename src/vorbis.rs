use crate::{huffman::HuffmanTree, util};
use bitstream_io::{BitRead, BitReader, LittleEndian};
use deku::prelude::*;
use std::cmp::Ordering;
use std::io::Cursor;

#[derive(Debug, DekuRead)]
pub struct VorbisPacket {
    packet_type: u8,
    #[deku(assert_eq = "*b\"vorbis\"")]
    magic: [u8; 6],
    #[deku(ctx = "*packet_type")]
    pub packet: VorbisPacketType,
}

#[derive(Debug, DekuRead)]
#[deku(ctx = "id: u8", id = "id")]
pub enum VorbisPacketType {
    #[deku(id = "0")]
    Audio(Audio),
    #[deku(id = "1")]
    Identification(IdHeader),
    #[deku(id = "3")]
    Comment(CommentHeader),
    #[deku(id = "5")]
    Setup(u8),
}

#[derive(Debug, DekuRead)]
pub struct IdHeader {
    #[deku(assert_eq = "0")]
    vorbis_version: u32,
    audio_channels: u8,
    audio_sample_rate: u32,
    bitrate_maximum: i32,
    bitrate_nominal: i32,
    bitrate_minimum: i32,
    #[deku(bits = 4, map = "|x: u16| -> Result<_, DekuError> { Ok(1 << x) }")]
    blocksize_1: u16, // The blocksizes are reversed because deku can't handle reading bits in the correct order!
    #[deku(bits = 4, map = "|x: u16| -> Result<_, DekuError> { Ok(1 << x) }")]
    blocksize_0: u16,
    //#[deku(bits = 1)] // Disabled to read a whole byte instead of 1 bit because deku can't handle reading bits in the correct order!
    framing_flag: bool,
}

impl IdHeader {
    pub fn is_valid(&self) -> bool {
        self.vorbis_version == 0
            && self.audio_channels > 0
            && self.audio_sample_rate > 0
            && 64 <= self.blocksize_0
            && self.blocksize_0 <= 8192
            && 64 <= self.blocksize_1
            && self.blocksize_1 <= 8192
            && self.blocksize_0 <= self.blocksize_1
            && self.framing_flag == true
    }
}

#[derive(Debug, DekuRead)]
pub struct CommentHeader {
    vendor_length: u32,
    #[deku(
        count = "vendor_length",
        map = "|x: &[u8]| -> Result<_, DekuError> { Ok(String::from_utf8_lossy(x).into_owned()) }"
    )]
    vendor_string: String,
    user_comment_list_length: u32,
    #[deku(count = "user_comment_list_length")]
    user_comments: Vec<UserComment>,
    //#[deku(bits = 1)] // Disabled to read a whole byte instead of 1 bit because deku can't handle reading bits in the correct order!
    framing_bit: bool,
}

impl CommentHeader {
    pub fn is_valid(&self) -> bool {
        self.framing_bit == true
    }
}

#[derive(Debug, DekuRead)]
pub struct UserComment {
    length: u32,
    #[deku(
        count = "length",
        map = "|x: &[u8]| -> Result<_, DekuError> { Ok(String::from_utf8_lossy(x).into_owned()) }"
    )]
    comment: String,
}

pub struct SetupHeader {
    // TODO: have a Raw struct with all the intermediate data from decoding, that can be moved into a non-raw struct with just the relevant stuff?
    vorbis_codebook_count: u8, // TODO: should this be removed? Maybe codebooks.len() is sufficient.
    codebooks: Vec<Codebook>,
}

impl SetupHeader {
    pub fn from_bytes(bytes: &[u8]) -> Self {
        let mut cursor = Cursor::new(bytes);
        let mut reader = BitReader::endian(&mut cursor, LittleEndian);

        // Codebook decode
        let vorbis_codebook_count: u8 = reader.read::<u8>(8).unwrap() + 1;
        let mut codebooks: Vec<Codebook> = Vec::new();
        for _ in 0..vorbis_codebook_count {
            let codebook = Codebook::decode(&mut reader);
            codebooks.push(codebook);
        }

        todo!()
    }
}

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
    fn decode<R, E>(reader: &mut BitReader<R, E>) -> Self
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
            _ => panic!("Reserved codebook lookup type: {}", lookup_type),
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

#[derive(Debug, DekuRead)]
pub struct Audio {
    tmp: u8,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_() {}
}
