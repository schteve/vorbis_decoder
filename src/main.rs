#![allow(dead_code)]
#![forbid(unsafe_code)]

mod ogg;

//use bitstream_io::{BitRead, BitReader, LittleEndian};
use deku::prelude::*;
use ogg::OggPage;
//use std::io::{Cursor, Read};

fn main() {
    let bytes = include_bytes!("../bin/frampton.ogg");
    //let mut cursor = Cursor::new(&bytes);
    //let mut reader = BitReader::endian(&mut cursor, LittleEndian);

    let ogg_page = OggPage::from_bytes((bytes, 0)).unwrap().1;
    dbg!(&ogg_page);

    // Check decode
    let to_bytes = ogg_page.to_bytes().unwrap();
    assert_eq!(bytes[0..to_bytes.len()].to_vec(), to_bytes);

    // Check CRC
    assert!(ogg_page.verify_crc());
}
