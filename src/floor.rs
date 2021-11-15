use bitstream_io::{BitRead, BitReader};

#[derive(Debug)]
pub enum Floor {
    Zero(Floor0),
    One(Floor1),
}

impl Floor {
    pub fn decode<R, E>(reader: &mut BitReader<R, E>) -> Self
    where
        R: std::io::Read,
        E: bitstream_io::Endianness,
    {
        let vorbis_floor_type = reader.read::<u16>(16).unwrap();
        match vorbis_floor_type {
            0 => Self::Zero(Floor0::decode(reader)),
            1 => Self::One(Floor1::decode(reader)),
            x => panic!("Invalid floor type {}", x),
        }
    }
}

#[derive(Debug, Default)]
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
    pub fn decode<R, E>(reader: &mut BitReader<R, E>) -> Self
    where
        R: std::io::Read,
        E: bitstream_io::Endianness,
    {
        let order = reader.read(8).unwrap();
        let rate = reader.read(16).unwrap();
        let bark_map_size = reader.read(16).unwrap();
        let amplitude_bits = reader.read(6).unwrap();
        let amplitude_offset = reader.read(8).unwrap();
        let number_of_books = reader.read::<u8>(4).unwrap() + 1;
        let book_list = (0..number_of_books)
            .map(|_| reader.read(8).unwrap())
            .collect();
        // TODO: verify that all elements are <= the maximum codebook number

        Self {
            order,
            rate,
            bark_map_size,
            amplitude_bits,
            amplitude_offset,
            number_of_books,
            book_list,
        }
    }
}

#[derive(Debug, Default)]
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
    pub fn decode<R, E>(reader: &mut BitReader<R, E>) -> Self
    where
        R: std::io::Read,
        E: bitstream_io::Endianness,
    {
        let partitions = reader.read(5).unwrap();
        let partition_class_list: Vec<u8> =
            (0..partitions).map(|_| reader.read(4).unwrap()).collect();
        let maximum_class = partition_class_list.iter().copied().max().unwrap();

        let mut classes: Vec<Class> = Vec::new();
        for _ in 0..=maximum_class {
            let dimensions = reader.read::<u8>(3).unwrap() + 1;
            let subclasses = reader.read(2).unwrap();
            let masterbooks = if subclasses > 0 {
                // TODO: validate that this element is not greater than the highest numbered codebook
                Some(reader.read(8).unwrap())
            } else {
                None
            };
            let max = 2_u8.pow(subclasses as u32);
            let subclass_books: Vec<i32> = (0..max)
                .map(|_| reader.read::<i32>(8).unwrap() - 1) // TODO: validate that this element is not greater than the highest numbered codebook
                .collect(); // TODO: spec says this is an unsigned integer; but what to do if its value is zero before subtracting? Treat as -1 or wrap to 0xFF (or 0xFFFFFFFF)?

            classes.push(Class {
                dimensions,
                subclasses,
                masterbooks,
                subclass_books,
            });
        }

        let multiplier = reader.read::<u8>(2).unwrap() + 1;
        let rangebits = reader.read(4).unwrap();
        let mut x_list: Vec<u32> = vec![0, 2u32.pow(rangebits as u32)];
        for &current_class_number in &partition_class_list {
            let max = classes[current_class_number as usize].dimensions;
            assert_ne!(max, 0);
            x_list.extend((0..max).map(|_| reader.read::<u32>(rangebits as u32).unwrap()));
        }
        assert!(x_list.len() <= 65);
        // TODO: validate that all element values in x_list are unique within the vector

        Self {
            partitions,
            partition_class_list,
            maximum_class,
            classes,
            multiplier,
            rangebits,
            x_list,
        }
    }
}

#[derive(Debug, Default)]
pub struct Class {
    dimensions: u8,
    subclasses: u8,
    masterbooks: Option<u8>,
    subclass_books: Vec<i32>,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_() {}
}
