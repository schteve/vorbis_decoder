use crate::util;
use bitstream_io::{BitRead, BitReader};

#[derive(Debug)]
pub struct Mapping {
    mapping_type: u16,
    submaps: u8,
    coupling_steps: u8,
    magnitude: Vec<u8>,
    angle: Vec<u8>,
    mux: Vec<u8>,
    submaps_vec: Vec<Submap>,
}

impl Mapping {
    pub fn decode<R, E>(reader: &mut BitReader<R, E>) -> Self
    where
        R: std::io::Read,
        E: bitstream_io::Endianness,
    {
        // TODO: use audio_channels from the ID header instead
        let audio_channels: u8 = 1;

        let mapping_type = reader.read(16).unwrap();
        assert_eq!(mapping_type, 0);

        let flag: bool = reader.read::<u8>(1).unwrap() == 1;
        let submaps = if flag == true {
            reader.read::<u8>(4).unwrap() + 1
        } else {
            1
        };

        let flag: bool = reader.read::<u8>(1).unwrap() == 1;
        let mut magnitude: Vec<u8> = Vec::new();
        let mut angle: Vec<u8> = Vec::new();
        let coupling_steps = if flag == true {
            // Polar channel mapping is in use
            let coupling_steps = reader.read::<u8>(8).unwrap() + 1;
            for _ in 0..coupling_steps {
                let m_bits = util::ilog(audio_channels as i32 - 1);
                let m = reader.read::<u8>(m_bits).unwrap();
                let a_bits = util::ilog(audio_channels as i32 - 1);
                let a = reader.read::<u8>(a_bits).unwrap();

                // Validate:
                // If for any coupling step the angle channel number equals the magnitude channel number,
                // the magnitude channel number is greater than [audio_channels]-1,
                // or the angle channel is greater than [audio_channels]-1, the stream is undecodable.
                assert_ne!(m, a);
                assert!(m >= audio_channels);
                assert!(a >= audio_channels);

                magnitude.push(m);
                angle.push(a);
            }
            coupling_steps
        } else {
            0
        };

        let reserved: u8 = reader.read(2).unwrap();
        assert_eq!(reserved, 0);

        let mux: Vec<u8> = if submaps > 1 {
            // Read channel multiplex settings
            (0..audio_channels)
                .map(|_| {
                    let value = reader.read::<u8>(4).unwrap();
                    assert!(value < submaps);
                    value
                })
                .collect()
        } else {
            // This isn't specified as far as I can tell, but since other parts of the spec
            // assume a mux value exists for each audio channel then the default must be 0
            vec![0; audio_channels as usize]
        };

        // Read the floor and residue numbers for use in decoding each submap
        let submaps_vec = (0..submaps).map(|_| Submap::decode(reader)).collect();

        Self {
            mapping_type,
            submaps,
            coupling_steps,
            magnitude,
            angle,
            mux,
            submaps_vec,
        }
    }
}

#[derive(Debug)]
pub struct Submap {
    floor: u8,
    residue: u8,
}

impl Submap {
    pub fn decode<R, E>(reader: &mut BitReader<R, E>) -> Self
    where
        R: std::io::Read,
        E: bitstream_io::Endianness,
    {
        let _: u8 = reader.read(8).unwrap(); // Unused time configuration placeholder

        let floor = reader.read(8).unwrap();
        // TODO: verify the floor number is not greater than the highest number floor configured for the bitstream

        let residue = reader.read(8).unwrap();
        // TODO: verify the residue number is not greater than the highest number residue configured for the bitstream

        Self { floor, residue }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_() {}
}
