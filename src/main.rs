#![allow(clippy::bool_comparison)]
#![allow(clippy::needless_bool)]
#![allow(dead_code)]
#![forbid(unsafe_code)]

mod codebook;
mod floor;
mod huffman;
mod mapping;
mod mode;
mod ogg;
mod residue;
mod time_domain;
mod util;
mod vorbis;

//use bitstream_io::{BitRead, BitReader, LittleEndian};
use crate::ogg::OggPage;
use crate::vorbis::*;
use deku::prelude::*;
//use std::io::{Cursor, Read};

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
    dbg!(&ogg_page);
    let (ogg_data_remaining, id_header) = VorbisPacket::from_bytes((&ogg_page.data, 0)).unwrap();
    assert!(ogg_data_remaining.0.is_empty()); // Unclear if this is a spec requirement
    assert_eq!(ogg_data_remaining.1, 0);
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
    dbg!(&ogg_page);
    let (ogg_data_remaining, comment_header) =
        VorbisPacket::from_bytes((&ogg_page.data, 0)).unwrap();
    //dbg!(&comment_header);
    match comment_header.packet {
        VorbisPacketType::Comment(comment) => {
            dbg!(&comment);
            dbg!(comment.is_valid());
        }
        x => panic!("Expected comment header type, got {:?}", x),
    }

    // Setup header
    dbg!(&ogg_data_remaining); // Dev hack. Second Ogg page has both comment and setup headers
    let _setup_header = SetupHeader::from_bytes(ogg_data_remaining).unwrap();
}
