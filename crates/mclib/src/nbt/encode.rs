//! Encoding of Java Edition network NBT.

use crate::ProtocolError;
use crate::nbt::value::{MAX_DEPTH, Nbt, tag_of};

impl Nbt {
    /// Encodes a root tag and its payload, without a root name.
    ///
    /// # Errors
    ///
    /// Returns an error for mixed list types or excessive lengths or nesting.
    pub fn encode_network(&self) -> Result<Vec<u8>, ProtocolError> {
        let mut encoder = NbtEncoder {
            bytes: vec![tag_of(self)],
        };
        encoder.pack_payload(self, 0)?;
        Ok(encoder.bytes)
    }
}

struct NbtEncoder {
    bytes: Vec<u8>,
}

impl NbtEncoder {
    fn pack_payload(&mut self, value: &Nbt, depth: usize) -> Result<(), ProtocolError> {
        if depth > MAX_DEPTH {
            return Err(ProtocolError::Overflow);
        }

        match value {
            Nbt::Byte(value) => self.pack_byte(*value),
            Nbt::Short(value) => self.pack_short(*value),
            Nbt::Int(value) => self.pack_int(*value),
            Nbt::Long(value) => self.pack_long(*value),
            Nbt::Float(value) => self.pack_float(*value),
            Nbt::Double(value) => self.pack_double(*value),
            Nbt::ByteArray(values) => self.pack_byte_array(values)?,
            Nbt::String(value) => self.pack_string(value)?,
            Nbt::List(items) => self.pack_list(items, depth)?,
            Nbt::Compound(entries) => self.pack_compound(entries, depth)?,
            Nbt::IntArray(values) => self.pack_int_array(values)?,
            Nbt::LongArray(values) => self.pack_long_array(values)?,
        }
        Ok(())
    }

    fn pack_byte(&mut self, value: i8) {
        self.bytes.push(value.cast_unsigned());
    }

    fn pack_short(&mut self, value: i16) {
        self.bytes.extend_from_slice(&value.to_be_bytes());
    }

    fn pack_int(&mut self, value: i32) {
        self.bytes.extend_from_slice(&value.to_be_bytes());
    }

    fn pack_long(&mut self, value: i64) {
        self.bytes.extend_from_slice(&value.to_be_bytes());
    }

    fn pack_float(&mut self, value: f32) {
        self.bytes.extend_from_slice(&value.to_be_bytes());
    }

    fn pack_double(&mut self, value: f64) {
        self.bytes.extend_from_slice(&value.to_be_bytes());
    }

    fn pack_byte_array(&mut self, values: &[u8]) -> Result<(), ProtocolError> {
        let length = i32::try_from(values.len()).map_err(|_| ProtocolError::Overflow)?;
        self.pack_int(length);
        self.bytes.extend_from_slice(values);
        Ok(())
    }

    fn pack_string(&mut self, value: &str) -> Result<(), ProtocolError> {
        let mut encoded = Vec::new();
        for unit in value.encode_utf16() {
            match unit {
                1..=0x7f => encoded.push((unit & 0x7f) as u8),
                0..=0x7ff => {
                    encoded.push(0xc0 | ((unit >> 6) & 0x1f) as u8);
                    encoded.push(0x80 | (unit & 0x3f) as u8);
                }
                _ => {
                    encoded.push(0xe0 | (unit >> 12) as u8);
                    encoded.push(0x80 | ((unit >> 6) & 0x3f) as u8);
                    encoded.push(0x80 | (unit & 0x3f) as u8);
                }
            }
            if encoded.len() > usize::from(u16::MAX) {
                return Err(ProtocolError::Overflow);
            }
        }
        let length = u16::try_from(encoded.len()).map_err(|_| ProtocolError::Overflow)?;
        self.bytes.extend_from_slice(&length.to_be_bytes());
        self.bytes.extend(encoded);
        Ok(())
    }

    fn pack_list(&mut self, items: &[Nbt], depth: usize) -> Result<(), ProtocolError> {
        let tag = items.first().map_or(0, tag_of);
        if items.iter().any(|item| tag_of(item) != tag) {
            return Err(ProtocolError::InvalidData);
        }

        let length = i32::try_from(items.len()).map_err(|_| ProtocolError::Overflow)?;
        self.bytes.push(tag);
        self.pack_int(length);
        for item in items {
            self.pack_payload(item, depth + 1)?;
        }
        Ok(())
    }

    fn pack_compound(
        &mut self,
        entries: &[(String, Nbt)],
        depth: usize,
    ) -> Result<(), ProtocolError> {
        for (name, value) in entries {
            self.bytes.push(tag_of(value));
            self.pack_string(name)?;
            self.pack_payload(value, depth + 1)?;
        }
        self.bytes.push(0);
        Ok(())
    }

    fn pack_int_array(&mut self, values: &[i32]) -> Result<(), ProtocolError> {
        let length = i32::try_from(values.len()).map_err(|_| ProtocolError::Overflow)?;
        self.pack_int(length);
        for value in values {
            self.pack_int(*value);
        }
        Ok(())
    }

    fn pack_long_array(&mut self, values: &[i64]) -> Result<(), ProtocolError> {
        let length = i32::try_from(values.len()).map_err(|_| ProtocolError::Overflow)?;
        self.pack_int(length);
        for value in values {
            self.pack_long(*value);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_compound_includes_root_tag_and_end() {
        assert_eq!(Nbt::Compound(Vec::new()).encode_network().unwrap(), [10, 0]);
    }

    #[test]
    fn lists_use_one_tag_and_four_byte_length() {
        let value = Nbt::List(vec![Nbt::Int(1), Nbt::Int(-1)]);
        assert_eq!(
            value.encode_network().unwrap(),
            [9, 3, 0, 0, 0, 2, 0, 0, 0, 1, 0xff, 0xff, 0xff, 0xff]
        );
        assert_eq!(
            Nbt::List(Vec::new()).encode_network().unwrap(),
            [9, 0, 0, 0, 0, 0]
        );
    }

    #[test]
    fn rejects_mixed_list_types() {
        let value = Nbt::List(vec![Nbt::Int(1), Nbt::Byte(2)]);
        assert!(matches!(
            value.encode_network(),
            Err(ProtocolError::InvalidData)
        ));
    }

    #[test]
    fn rejects_excessive_nesting() {
        let mut value = Nbt::Int(1);
        for _ in 0..=MAX_DEPTH {
            value = Nbt::List(vec![value]);
        }
        assert!(matches!(
            value.encode_network(),
            Err(ProtocolError::Overflow)
        ));
    }
    #[test]
    fn matches_modified_utf8_bytes() {
        for (text, payload) in [
            ("", vec![0, 0]),
            ("a\0é", vec![0, 5, 0x61, 0xc0, 0x80, 0xc3, 0xa9]),
            ("😀", vec![0, 6, 0xed, 0xa0, 0xbd, 0xed, 0xb8, 0x80]),
            ("ࠀ", vec![0, 3, 0xe0, 0xa0, 0x80]),
            ("\u{7f}", vec![0, 1, 0x7f]),
            ("\u{80}", vec![0, 2, 0xc2, 0x80]),
            ("\u{7ff}", vec![0, 2, 0xdf, 0xbf]),
            ("\u{ffff}", vec![0, 3, 0xef, 0xbf, 0xbf]),
        ] {
            let value = Nbt::String(text.to_owned());
            let mut bytes = vec![8];
            bytes.extend(payload);
            assert_eq!(value.encode_network().unwrap(), bytes);
            assert_eq!(Nbt::decode_network(&mut bytes.as_slice()).unwrap(), value);
        }
    }

    #[test]
    fn enforces_encoded_byte_limit() {
        let value = "a".repeat(usize::from(u16::MAX));
        let bytes = Nbt::String(value.clone()).encode_network().unwrap();
        assert_eq!(&bytes[..3], &[8, 0xff, 0xff]);
        assert_eq!(
            Nbt::decode_network(&mut bytes.as_slice()).unwrap(),
            Nbt::String(value.clone())
        );
        assert!(matches!(
            Nbt::String(value + "a").encode_network(),
            Err(ProtocolError::Overflow)
        ));
        // NUL uses two bytes per UTF-16 unit.
        assert!(matches!(
            Nbt::String("\0".repeat(32_768)).encode_network(),
            Err(ProtocolError::Overflow)
        ));
    }
}
