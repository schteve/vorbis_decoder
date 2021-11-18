use crate::util;
use bitstream_io::{BitRead, BitReader};
use thiserror::Error;

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
    pub fn decode<R, E>(reader: &mut BitReader<R, E>) -> Result<Self, MappingError>
    where
        R: std::io::Read,
        E: bitstream_io::Endianness,
    {
        // TODO: use audio_channels from the ID header instead
        let audio_channels: u8 = 1;

        let mapping_type = reader.read(16)?;
        if mapping_type != 0 {
            return Err(MappingError::InvalidMappingType(mapping_type));
        }

        let flag: bool = reader.read::<u8>(1)? == 1;
        let submaps = if flag == true {
            reader.read::<u8>(4)? + 1
        } else {
            1
        };

        let flag: bool = reader.read::<u8>(1)? == 1;
        let mut magnitude: Vec<u8> = Vec::new();
        let mut angle: Vec<u8> = Vec::new();
        let coupling_steps = if flag == true {
            // Polar channel mapping is in use
            let coupling_steps = reader.read::<u8>(8)? + 1;
            for _ in 0..coupling_steps {
                let m_bits = util::ilog(audio_channels as i32 - 1);
                let m = reader.read::<u8>(m_bits)?;
                let a_bits = util::ilog(audio_channels as i32 - 1);
                let a = reader.read::<u8>(a_bits)?;

                // Validate:
                // If for any coupling step the angle channel number equals the magnitude channel number,
                // the magnitude channel number is greater than [audio_channels]-1,
                // or the angle channel is greater than [audio_channels]-1, the stream is undecodable.
                if m == a {
                    return Err(MappingError::PolarAngEqualsMag(m, a));
                }
                if m >= audio_channels {
                    return Err(MappingError::PolarMagInvalid(m));
                }
                if a >= audio_channels {
                    return Err(MappingError::PolarAngInvalid(a));
                }

                magnitude.push(m);
                angle.push(a);
            }
            coupling_steps
        } else {
            0
        };

        let reserved: u8 = reader.read(2)?;
        if reserved != 0 {
            return Err(MappingError::Reserved(reserved));
        }

        let mux: Vec<u8> = if submaps > 1 {
            // Read channel multiplex settings
            (0..audio_channels)
                .map(|_| {
                    let value = reader.read::<u8>(4)?;
                    if value >= submaps {
                        return Err(MappingError::MuxInvalid(value));
                    }
                    Ok(value)
                })
                .collect::<Result<_, _>>()?
        } else {
            // This isn't specified as far as I can tell, but since other parts of the spec
            // assume a mux value exists for each audio channel then the default must be 0
            vec![0; audio_channels as usize]
        };

        // Read the floor and residue numbers for use in decoding each submap
        let submaps_vec = (0..submaps)
            .map(|_| Submap::decode(reader))
            .collect::<Result<_, _>>()?;

        Ok(Self {
            mapping_type,
            submaps,
            coupling_steps,
            magnitude,
            angle,
            mux,
            submaps_vec,
        })
    }
}

#[derive(Debug)]
pub struct Submap {
    floor: u8,
    residue: u8,
}

impl Submap {
    pub fn decode<R, E>(reader: &mut BitReader<R, E>) -> Result<Self, MappingError>
    where
        R: std::io::Read,
        E: bitstream_io::Endianness,
    {
        let _: u8 = reader.read(8)?; // Unused time configuration placeholder

        let floor = reader.read(8)?;
        // TODO: verify the floor number is not greater than the highest number floor configured for the bitstream

        let residue = reader.read(8)?;
        // TODO: verify the residue number is not greater than the highest number residue configured for the bitstream

        Ok(Self { floor, residue })
    }
}

#[derive(Debug, Error)]
pub enum MappingError {
    #[error("Invalid mapping type: {0}")]
    InvalidMappingType(u16),

    #[error("Polar channel mapping angle {0} equals magnitude {1}")]
    PolarAngEqualsMag(u8, u8),

    #[error("Polar magnitude channel {0} is greater than number of channels")]
    PolarMagInvalid(u8),

    #[error("Polar angle channel {0} is greater than number of channels")]
    PolarAngInvalid(u8),

    #[error("Reserved value invalid: {0}")]
    Reserved(u8),

    #[error("Mux {0} is greater than highest submap")]
    MuxInvalid(u8),

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
