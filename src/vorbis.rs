use crate::{
    codebook::Codebook, floor::Floor, mapping::Mapping, mode::Mode, residue::Residue,
    time_domain::TimeDomainTransform,
};
use bitstream_io::{BitRead, BitReader, LittleEndian};
use deku::prelude::*;
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
    codebook_count: u8, // TODO: should this be removed? Maybe codebooks.len() is sufficient.
    codebooks: Vec<Codebook>,
    time_count: u8,
    time_domain_transforms: Vec<TimeDomainTransform>,
    floor_count: u8,
    floor_configurations: Vec<Floor>,
    residue_count: u8,
    residue_configurations: Vec<Residue>,
    mapping_count: u8,
    mapping_configurations: Vec<Mapping>,
    mode_count: u8,
    mode_configurations: Vec<Mode>,
    framing_flag: bool,
}

impl SetupHeader {
    pub fn from_bytes(input: (&[u8], usize)) -> Self {
        assert_eq!(input.1, 0); // Assume packet starts at bit 0
        let mut cursor = Cursor::new(input.0);
        let mut reader = BitReader::endian(&mut cursor, LittleEndian);

        // This is a hack since currently the vorbis packet's header is not decoded before calling this function
        let packet_type = reader.read::<u8>(8).unwrap();
        assert_eq!(packet_type, 5);
        let magic = [
            reader.read::<u8>(8).unwrap(),
            reader.read::<u8>(8).unwrap(),
            reader.read::<u8>(8).unwrap(),
            reader.read::<u8>(8).unwrap(),
            reader.read::<u8>(8).unwrap(),
            reader.read::<u8>(8).unwrap(),
        ];
        assert_eq!(&magic, b"vorbis");

        // Codebooks
        let codebook_count: u8 = reader.read::<u8>(8).unwrap() + 1;
        let codebooks = (0..codebook_count)
            .map(|_| Codebook::decode(&mut reader))
            .collect();

        // Time domain transforms
        let time_count = reader.read::<u8>(6).unwrap() + 1;
        let time_domain_transforms = (0..time_count)
            .map(|_| TimeDomainTransform::decode(&mut reader))
            .collect();

        // Floors
        let floor_count = reader.read::<u8>(6).unwrap() + 1;
        let floor_configurations = (0..floor_count)
            .map(|_| Floor::decode(&mut reader))
            .collect();

        // Residues
        let residue_count = reader.read::<u8>(6).unwrap() + 1;
        let residue_configurations = (0..residue_count)
            .map(|_| Residue::decode(&mut reader))
            .collect();

        // Mappings
        let mapping_count = reader.read::<u8>(6).unwrap() + 1;
        let mapping_configurations = (0..mapping_count)
            .map(|_| Mapping::decode(&mut reader))
            .collect();

        // Modes
        let mode_count = reader.read::<u8>(6).unwrap() + 1;
        let mode_configurations = (0..mode_count).map(|_| Mode::decode(&mut reader)).collect();
        let framing_flag: bool = reader.read::<u8>(1).unwrap() == 1;
        assert!(framing_flag);

        // Check post-conditions since we're not properly handling packet continuation
        let _ = reader.into_reader(); // Discard the reader
        let pos = cursor.position();
        assert_eq!(cursor.into_inner().len(), pos as usize); // Check that cursor made it through the entire underlying buffer - no data left

        Self {
            codebook_count,
            codebooks,
            time_count,
            time_domain_transforms,
            floor_count,
            floor_configurations,
            residue_count,
            residue_configurations,
            mapping_count,
            mapping_configurations,
            mode_count,
            mode_configurations,
            framing_flag,
        }
    }
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
