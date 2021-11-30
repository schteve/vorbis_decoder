use bitstream_io::{BitRead, BitReader};
use thiserror::Error;

#[derive(Debug, PartialEq)]
pub enum Floor {
    Zero(Floor0),
    One(Floor1),
}

impl Floor {
    pub fn decode<R, E>(reader: &mut BitReader<R, E>) -> Result<Self, FloorError>
    where
        R: std::io::Read,
        E: bitstream_io::Endianness,
    {
        let vorbis_floor_type = reader.read::<u16>(16)?;
        let floor = match vorbis_floor_type {
            0 => Self::Zero(Floor0::decode(reader)?),
            1 => Self::One(Floor1::decode(reader)?),
            x => return Err(FloorError::InvalidFloorType(x)),
        };
        Ok(floor)
    }
}

#[derive(Debug, Default, PartialEq)]
pub struct Floor0 {
    order: u8,
    rate: u16,
    bark_map_size: u16,
    amplitude_bits: u8,
    amplitude_offset: u8,
    number_of_books: u8,
    book_list: Vec<u8>,
}

impl Floor0 {
    pub fn decode<R, E>(reader: &mut BitReader<R, E>) -> Result<Self, FloorError>
    where
        R: std::io::Read,
        E: bitstream_io::Endianness,
    {
        let order = reader.read(8)?;
        let rate = reader.read(16)?;
        let bark_map_size = reader.read(16)?;
        let amplitude_bits = reader.read(6)?;
        let amplitude_offset = reader.read(8)?;
        let number_of_books = reader.read::<u8>(4)? + 1;
        let book_list = (0..number_of_books)
            .map(|_| reader.read(8))
            .collect::<Result<_, _>>()?;
        // TODO: verify that all elements are <= the maximum codebook number

        Ok(Self {
            order,
            rate,
            bark_map_size,
            amplitude_bits,
            amplitude_offset,
            number_of_books,
            book_list,
        })
    }
}

#[derive(Debug, Default, PartialEq)]
pub struct Floor1 {
    partitions: u8,
    partition_class_list: Vec<u8>,
    maximum_class: u8,
    classes: Vec<Class>,
    multiplier: u8,
    rangebits: u8,
    x_list: Vec<u32>,
}

impl Floor1 {
    pub fn decode<R, E>(reader: &mut BitReader<R, E>) -> Result<Self, FloorError>
    where
        R: std::io::Read,
        E: bitstream_io::Endianness,
    {
        let partitions = reader.read(5)?;
        let partition_class_list: Vec<u8> = (0..partitions)
            .map(|_| reader.read(4))
            .collect::<Result<_, _>>()?;
        let maximum_class = partition_class_list.iter().copied().max().unwrap();

        let mut classes: Vec<Class> = Vec::new();
        for _ in 0..=maximum_class {
            let dimensions = reader.read::<u8>(3)? + 1;
            let subclasses = reader.read(2)?;
            let masterbooks = if subclasses > 0 {
                // TODO: validate that this element is not greater than the highest numbered codebook
                Some(reader.read(8)?)
            } else {
                None
            };
            let max = 2_u8.pow(subclasses as u32);
            let subclass_books: Vec<i32> = (0..max)
                .map(|_| reader.read::<i32>(8).map(|i| i - 1)) // TODO: validate that this element is not greater than the highest numbered codebook
                .collect::<Result<_, _>>()?; // TODO: spec says this is an unsigned integer; but what to do if its value is zero before subtracting? Treat as -1 or wrap to 0xFF (or 0xFFFFFFFF)?

            classes.push(Class {
                dimensions,
                subclasses,
                masterbooks,
                subclass_books,
            });
        }

        let multiplier = reader.read::<u8>(2)? + 1;
        let rangebits = reader.read(4)?;
        let mut x_list: Vec<u32> = vec![0, 2u32.pow(rangebits as u32)];
        for &current_class_number in &partition_class_list {
            let max = classes[current_class_number as usize].dimensions;
            assert_ne!(max, 0); // Should be impossible by design; dimensions is always a 3 bit number + 1
            for _ in 0..max {
                let val = reader.read::<u32>(rangebits as u32)?;
                x_list.push(val);
            }
        }

        if x_list.len() > 65 {
            return Err(FloorError::XListTooLong(x_list.len()));
        }
        // TODO: validate that all element values in x_list are unique within the vector

        Ok(Self {
            partitions,
            partition_class_list,
            maximum_class,
            classes,
            multiplier,
            rangebits,
            x_list,
        })
    }
}

#[derive(Debug, Default, PartialEq)]
pub struct Class {
    dimensions: u8,
    subclasses: u8,
    masterbooks: Option<u8>,
    subclass_books: Vec<i32>,
}

#[derive(Debug, Error)]
pub enum FloorError {
    #[error("Invalid floor type: {0}")]
    InvalidFloorType(u16),

    #[error("Floor X list too long: {0}")]
    XListTooLong(usize),

    // Represents all cases of `std::io::Error`.
    #[error(transparent)]
    IOError(#[from] std::io::Error),
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_floor_decode() {
        use bitstream_io::{BitReader, LittleEndian};
        use std::io::Cursor;

        // Frampton floor config 0 (type 1)
        let input = [
            1, 0, 6, 34, 100, 38, 16, 40, 128, 2, 3, 25, 0, 112, 128, 144, 32, 5, 0, 20, 22, 24,
            58, 134, 139, 128, 128, 92, 66, 70, 129, 65, 225, 152, 112, 78, 58, 109,
        ];
        let mut cursor = Cursor::new(input);
        let mut reader = BitReader::endian(&mut cursor, LittleEndian);
        let floor = Floor::decode(&mut reader).unwrap();
        assert_eq!(
            floor,
            Floor::One(Floor1 {
                partitions: 6,
                partition_class_list: vec![0, 1, 1, 2, 3, 3],
                maximum_class: 3,
                classes: vec![
                    Class {
                        dimensions: 2,
                        subclasses: 0,
                        masterbooks: None,
                        subclass_books: vec![3],
                    },
                    Class {
                        dimensions: 3,
                        subclasses: 1,
                        masterbooks: Some(0),
                        subclass_books: vec![4, 5],
                    },
                    Class {
                        dimensions: 3,
                        subclasses: 2,
                        masterbooks: Some(1),
                        subclass_books: vec![-1, 6, 7, 8],
                    },
                    Class {
                        dimensions: 3,
                        subclasses: 2,
                        masterbooks: Some(2),
                        subclass_books: vec![-1, 9, 10, 11],
                    },
                ],
                multiplier: 2,
                rangebits: 7,
                x_list: vec![
                    0, 128, 12, 46, 4, 8, 16, 23, 33, 70, 2, 6, 10, 14, 19, 28, 39, 58, 90
                ],
            })
        );

        // X List too long
        let input = [
            1, 0, 30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 34, 100, 38, 16, 40, 128, 2, 3, 25, 0,
            112, 128, 144, 32, 5, 0, 20, 22, 24, 58, 134, 139, 128, 128, 92, 66, 70, 129, 65, 225,
            152, 112, 78, 58, 109, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];
        let mut cursor = Cursor::new(input);
        let mut reader = BitReader::endian(&mut cursor, LittleEndian);
        let err = Floor::decode(&mut reader).unwrap_err();
        assert!(matches!(err, FloorError::XListTooLong(67)));

        // IOError
        let input = [];
        let mut cursor = Cursor::new(input);
        let mut reader = BitReader::endian(&mut cursor, LittleEndian);
        let err = Floor::decode(&mut reader).unwrap_err();
        match err {
            FloorError::IOError(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => (),
            x => panic!("Unexpected result: {:?}", x),
        }
    }
}
