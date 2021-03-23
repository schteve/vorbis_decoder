use bitstream_io::{BitRead, BitReader};

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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_() {}
}
