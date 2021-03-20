#![allow(clippy::bool_comparison)]
#![allow(clippy::needless_bool)]
#![allow(dead_code)]
#![forbid(unsafe_code)]

mod ogg;
mod vorbis;

//use bitstream_io::{BitRead, BitReader, LittleEndian};
use deku::prelude::*;
use ogg::OggPage;
//use std::io::{Cursor, Read};
use vorbis::*;

fn main() {
    let bytes = include_bytes!("../bin/frampton.ogg");
    //let mut cursor = Cursor::new(&bytes);
    //let mut reader = BitReader::endian(&mut cursor, LittleEndian);

    /*
    let ogg_page = OggPage::from_bytes((bytes, 0)).unwrap().1;
    //dbg!(&ogg_page);

    // Check decode
    let to_bytes = ogg_page.to_bytes().unwrap();
    assert_eq!(bytes[0..to_bytes.len()].to_vec(), to_bytes);

    // Check CRC
    assert!(ogg_page.verify_crc());
    */

    // Identification header
    let (input, ogg_page) = OggPage::from_bytes((bytes, 0)).unwrap();
    let id_header = VorbisPacket::from_bytes((&ogg_page.data, 0)).unwrap().1;
    //dbg!(&id_header);
    match id_header.packet {
        VorbisPacketType::Identification(id) => {
            dbg!(&id);
            dbg!(id.is_valid());
        }
        _ => panic!("Expected ID header type, got {:?}", id_header.packet),
    }

    // Comment header
    let (_input, ogg_page) = OggPage::from_bytes(input).unwrap();
    let comment_header = VorbisPacket::from_bytes((&ogg_page.data, 0)).unwrap().1;
    //dbg!(&comment_header);
    match comment_header.packet {
        VorbisPacketType::Comment(comment) => {
            dbg!(&comment);
            dbg!(comment.is_valid());
        }
        _ => panic!(
            "Expected comment header type, got {:?}",
            comment_header.packet
        ),
    }
}
