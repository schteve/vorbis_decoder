use bitstream_io::{BitRead, BitReader};

#[derive(Debug, Default)]
pub struct Floor1 {
    partitions: u8,
    partition_class_list: Vec<u8>,
    maximum_class: Option<u8>,
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
        let maximum_class = partition_class_list.iter().copied().max();

        let mut classes: Vec<Class> = Vec::new();
        if let Some(max) = maximum_class {
            for _ in 0..max {
                let dimensions = reader.read::<u8>(3).unwrap() + 1;
                let subclasses = reader.read(2).unwrap();
                let masterbooks = if subclasses > 0 {
                    // TODO: validate that this element is not greater than the highest numbered codebook
                    Some(reader.read(8).unwrap())
                } else {
                    None
                };
                let max = u8::pow(2, subclasses as u32);
                let subclass_books: Vec<u8> = (0..max)
                    .map(|_| {
                        let val: u8 = reader.read(8).unwrap();
                        assert_ne!(val, 0);
                        // TODO: validate that this element is not greater than the highest numbered codebook
                        val - 1
                    })
                    .collect();

                classes.push(Class {
                    dimensions,
                    subclasses,
                    masterbooks,
                    subclass_books,
                });
            }
        }

        let multiplier = reader.read::<u8>(2).unwrap() + 1;
        let rangebits = reader.read(4).unwrap();
        let mut x_list: Vec<u32> = Vec::new();
        x_list[0] = 0;
        x_list[1] = u32::pow(2, rangebits as u32);
        assert_ne!(partitions, 0);
        for i in 0..=partitions as usize {
            let current_class_number = partition_class_list[i] as usize;
            let max = classes[current_class_number].dimensions;
            assert_ne!(max, 0);
            x_list.extend((0..=max).map(|_| reader.read::<u32>(rangebits as u32).unwrap()));
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
    subclass_books: Vec<u8>,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_() {}
}
