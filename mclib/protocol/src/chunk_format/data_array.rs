pub struct DataArray(Vec<i64>);

impl From<Vec<i64>> for DataArray {
    fn from(value: Vec<i64>) -> Self {
        Self(value)
    }
}

impl DataArray {
    pub fn pack(&mut self, bits_per_entry: u8) -> Vec<i64> {
        let mut result = Vec::new();
        let max_chunk_size = 64 / bits_per_entry;

        for chunk in self.0.chunks(max_chunk_size as usize) {
            let mut number = 0;
            for (entry_index, entry) in chunk.iter().enumerate() {
                let bit_index = entry_index * bits_per_entry as usize;
                number |= entry << bit_index;
            }
            result.push(number);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::chunk_format::data_array::DataArray;

    #[test]
    fn test_data_array() {
        assert_eq!(
            DataArray::from(vec![
                1, 2, 2, 3, 4, 4, 5, 6, 6, 4, 8, 0, 7, 4, 3, 13, 15, 16, 9, 14, 10, 12, 0, 2,
            ])
            .pack(5),
            vec![0x0020863148418841, 0x01018A7260F68C87]
        );
    }
}
