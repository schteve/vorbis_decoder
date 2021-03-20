use crc_any::CRCu32;
use deku::prelude::*;

#[derive(Debug, DekuRead, DekuWrite)]
struct HeaderTypeFlag(u8);

impl HeaderTypeFlag {
    fn is_valid(&self) -> bool {
        (self.0 & !0x07) == 0
    }

    fn is_continued_packet(&self) -> bool {
        (self.0 & 0x01) != 0
    }

    fn is_first_page(&self) -> bool {
        (self.0 & 0x02) != 0
    }

    fn is_last_page(&self) -> bool {
        (self.0 & 0x04) != 0
    }
}

#[derive(Debug, DekuRead, DekuWrite)]
#[deku(magic = b"OggS")]
pub struct OggPage {
    //capture_pattern: u32,
    stream_structure_version: u8,
    header_type_flag: HeaderTypeFlag,
    absolute_granule_position: u64,
    stream_serial_number: u32,
    page_sequence_no: u32,
    page_checksum: u32,
    page_segments: u8,
    #[deku(count = "page_segments")]
    segment_table: Vec<u8>,
    #[deku(count = "segment_table.iter().map(|b| *b as usize).sum::<usize>()")]
    pub data: Vec<u8>,
}

impl OggPage {
    pub fn verify_crc(&self) -> bool {
        let mut bytes = self.to_bytes().expect("OggPage DekuWrite failed!");
        bytes[22] = 0;
        bytes[23] = 0;
        bytes[24] = 0;
        bytes[25] = 0;

        let mut crc32 = CRCu32::create_crc(0x04c11db7, 32, 0, 0, false);
        crc32.digest(&bytes);
        let crc = crc32.get_crc();
        crc == self.page_checksum
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_header_type_flag() {
        let flag = HeaderTypeFlag::from_bytes((&[0x00], 0)).unwrap().1;
        assert_eq!(flag.is_valid(), true);
        assert_eq!(flag.is_continued_packet(), false);
        assert_eq!(flag.is_first_page(), false);
        assert_eq!(flag.is_last_page(), false);

        let flag = HeaderTypeFlag::from_bytes((&[0xF], 0)).unwrap().1;
        assert_eq!(flag.is_valid(), false);
        assert_eq!(flag.is_continued_packet(), true);
        assert_eq!(flag.is_first_page(), true);
        assert_eq!(flag.is_last_page(), true);

        let flag = HeaderTypeFlag::from_bytes((&[0x5], 0)).unwrap().1;
        assert_eq!(flag.is_valid(), true);
        assert_eq!(flag.is_continued_packet(), true);
        assert_eq!(flag.is_first_page(), false);
        assert_eq!(flag.is_last_page(), true);
    }

    #[test]
    fn test_ogg_page_verify_crc() {
        let mut raw_bytes = vec![
            0x4F, 0x67, 0x67, 0x53, 0x00, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x4B, 0x86, 0x5C, 0x7D, 0x00, 0x00, 0x00, 0x00, 0xC1, 0xE3, 0xE7, 0xEF, 0x01, 0x1E,
            0x01, 0x76, 0x6F, 0x72, 0x62, 0x69, 0x73, 0x00, 0x00, 0x00, 0x00, 0x01, 0x44, 0xAC,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x77, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00,
            0xB8, 0x01,
        ];
        let ogg_page = OggPage::from_bytes((&raw_bytes, 0)).unwrap().1;
        assert_eq!(ogg_page.verify_crc(), true);

        raw_bytes[22] = 0;
        let ogg_page = OggPage::from_bytes((&raw_bytes, 0)).unwrap().1;
        assert_eq!(ogg_page.verify_crc(), false);
    }
}
