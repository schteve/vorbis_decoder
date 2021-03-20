use deku::prelude::*;

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
    Setup(SetupHeader),
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

#[derive(Debug, DekuRead)]
pub struct SetupHeader {
    tmp: u8,
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
