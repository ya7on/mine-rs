//! Decoding of Java Edition network NBT.

use std::io::{ErrorKind, Read};

use crate::ProtocolError;
use crate::nbt::value::{MAX_DEPTH, Nbt};

impl Nbt {
    /// Decodes one root tag and its payload, without a root name.
    ///
    /// # Errors
    ///
    /// Returns an error for truncated data, invalid tags or excessive nesting.
    pub fn decode_network(source: &mut dyn Read) -> Result<Self, ProtocolError> {
        let mut tag = [0];
        source.read_exact(&mut tag)?;
        NbtDecoder { source }.unpack_payload(tag[0], 0)
    }
}

struct NbtDecoder<'a> {
    source: &'a mut dyn Read,
}

impl NbtDecoder<'_> {
    fn unpack_payload(&mut self, tag: u8, depth: usize) -> Result<Nbt, ProtocolError> {
        if depth > MAX_DEPTH {
            return Err(ProtocolError::Overflow);
        }

        Ok(match tag {
            1 => Nbt::Byte(self.unpack_byte()?),
            2 => Nbt::Short(self.unpack_short()?),
            3 => Nbt::Int(self.unpack_int()?),
            4 => Nbt::Long(self.unpack_long()?),
            5 => Nbt::Float(self.unpack_float()?),
            6 => Nbt::Double(self.unpack_double()?),
            7 => Nbt::ByteArray(self.unpack_byte_array()?),
            8 => Nbt::String(self.unpack_string()?),
            9 => Nbt::List(self.unpack_list(depth)?),
            10 => Nbt::Compound(self.unpack_compound(depth)?),
            11 => Nbt::IntArray(self.unpack_int_array()?),
            12 => Nbt::LongArray(self.unpack_long_array()?),
            _ => return Err(ProtocolError::InvalidData),
        })
    }

    fn unpack_byte(&mut self) -> Result<i8, ProtocolError> {
        let mut bytes = [0; 1];
        self.source.read_exact(&mut bytes)?;
        Ok(bytes[0].cast_signed())
    }

    fn unpack_short(&mut self) -> Result<i16, ProtocolError> {
        let mut bytes = [0; 2];
        self.source.read_exact(&mut bytes)?;
        Ok(i16::from_be_bytes(bytes))
    }

    fn unpack_int(&mut self) -> Result<i32, ProtocolError> {
        let mut bytes = [0; 4];
        self.source.read_exact(&mut bytes)?;
        Ok(i32::from_be_bytes(bytes))
    }

    fn unpack_long(&mut self) -> Result<i64, ProtocolError> {
        let mut bytes = [0; 8];
        self.source.read_exact(&mut bytes)?;
        Ok(i64::from_be_bytes(bytes))
    }

    fn unpack_float(&mut self) -> Result<f32, ProtocolError> {
        let mut bytes = [0; 4];
        self.source.read_exact(&mut bytes)?;
        Ok(f32::from_be_bytes(bytes))
    }

    fn unpack_double(&mut self) -> Result<f64, ProtocolError> {
        let mut bytes = [0; 8];
        self.source.read_exact(&mut bytes)?;
        Ok(f64::from_be_bytes(bytes))
    }

    fn unpack_byte_array(&mut self) -> Result<Vec<u8>, ProtocolError> {
        let length = usize::try_from(self.unpack_int()?).map_err(|_| ProtocolError::InvalidData)?;
        // Grow only as bytes arrive; a forged count must not allocate gigabytes.
        let mut bytes = Vec::new();
        (&mut *self.source)
            .take(u64::try_from(length).map_err(|_| ProtocolError::Overflow)?)
            .read_to_end(&mut bytes)?;
        if bytes.len() != length {
            return Err(std::io::Error::from(ErrorKind::UnexpectedEof).into());
        }
        Ok(bytes)
    }

    fn unpack_string(&mut self) -> Result<String, ProtocolError> {
        let mut length = [0; 2];
        self.source.read_exact(&mut length)?;
        let mut bytes = vec![0; usize::from(u16::from_be_bytes(length))];
        self.source.read_exact(&mut bytes)?;

        let mut units = Vec::new();
        let mut bytes = bytes.as_slice();
        while !bytes.is_empty() {
            match bytes {
                [first @ 1..=0x7f, rest @ ..] => {
                    units.push(u16::from(*first));
                    bytes = rest;
                }
                [first @ 0xc0..=0xdf, second @ 0x80..=0xbf, rest @ ..] => {
                    let unit = (u16::from(first & 0x1f) << 6) | u16::from(second & 0x3f);
                    if unit != 0 && unit < 0x80 {
                        return Err(ProtocolError::InvalidData);
                    }
                    units.push(unit);
                    bytes = rest;
                }
                [
                    first @ 0xe0..=0xef,
                    second @ 0x80..=0xbf,
                    third @ 0x80..=0xbf,
                    rest @ ..,
                ] => {
                    let unit = (u16::from(first & 0x0f) << 12)
                        | (u16::from(second & 0x3f) << 6)
                        | u16::from(third & 0x3f);
                    if unit < 0x800 {
                        return Err(ProtocolError::InvalidData);
                    }
                    units.push(unit);
                    bytes = rest;
                }
                _ => return Err(ProtocolError::InvalidData),
            }
        }
        String::from_utf16(&units).map_err(|_| ProtocolError::InvalidData)
    }

    fn unpack_list(&mut self, depth: usize) -> Result<Vec<Nbt>, ProtocolError> {
        let mut tag = [0];
        self.source.read_exact(&mut tag)?;
        let length = self.unpack_int()?;
        if length <= 0 {
            return Ok(Vec::new());
        }
        if !(1..=12).contains(&tag[0]) {
            return Err(ProtocolError::InvalidData);
        }

        let length = usize::try_from(length).map_err(|_| ProtocolError::Overflow)?;
        let mut items = Vec::with_capacity(length.min(1024));
        for _ in 0..length {
            items.push(self.unpack_payload(tag[0], depth + 1)?);
        }
        Ok(items)
    }

    fn unpack_compound(&mut self, depth: usize) -> Result<Vec<(String, Nbt)>, ProtocolError> {
        let mut entries = Vec::new();
        loop {
            let mut tag = [0];
            self.source.read_exact(&mut tag)?;
            if tag[0] == 0 {
                break;
            }
            if tag[0] > 12 {
                return Err(ProtocolError::InvalidData);
            }
            let name = self.unpack_string()?;
            let value = self.unpack_payload(tag[0], depth + 1)?;
            entries.push((name, value));
        }
        Ok(entries)
    }

    fn unpack_int_array(&mut self) -> Result<Vec<i32>, ProtocolError> {
        let length = usize::try_from(self.unpack_int()?).map_err(|_| ProtocolError::InvalidData)?;
        let mut values = Vec::with_capacity(length.min(1024));
        for _ in 0..length {
            values.push(self.unpack_int()?);
        }
        Ok(values)
    }

    fn unpack_long_array(&mut self) -> Result<Vec<i64>, ProtocolError> {
        let length = usize::try_from(self.unpack_int()?).map_err(|_| ProtocolError::InvalidData)?;
        let mut values = Vec::with_capacity(length.min(1024));
        for _ in 0..length {
            values.push(self.unpack_long()?);
        }
        Ok(values)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decodes_a_known_network_compound() {
        let bytes = [10, 3, 0, 1, b'x', 0, 0, 0, 42, 0];
        let value = Nbt::compound(vec![("x", Nbt::Int(42))]);
        assert_eq!(Nbt::decode_network(&mut bytes.as_slice()).unwrap(), value);
        assert_eq!(value.encode_network().unwrap(), bytes);
    }

    #[test]
    fn roundtrips_every_tag() {
        for value in [
            Nbt::Byte(-1),
            Nbt::Short(i16::MIN),
            Nbt::Int(i32::MIN),
            Nbt::Long(i64::MIN),
            Nbt::Float(-1.5),
            Nbt::Double(1.25),
            Nbt::ByteArray(vec![0, 128, 255]),
            Nbt::String("a\0😀".to_owned()),
            Nbt::List(vec![Nbt::Compound(Vec::new())]),
            Nbt::compound(vec![
                ("", Nbt::Int(0)),
                ("a\0😀", Nbt::String("é".to_owned())),
            ]),
            Nbt::IntArray(vec![i32::MIN, 0, i32::MAX]),
            Nbt::LongArray(vec![i64::MIN, 0, i64::MAX]),
        ] {
            let bytes = value.encode_network().unwrap();
            assert_eq!(Nbt::decode_network(&mut bytes.as_slice()).unwrap(), value);
        }
    }

    #[test]
    fn matches_fixed_width_scalar_bytes() {
        for (bytes, value) in [
            (vec![1, 0xff], Nbt::Byte(-1)),
            (vec![2, 0x80, 0], Nbt::Short(i16::MIN)),
            (vec![3, 0x80, 0, 0, 0], Nbt::Int(i32::MIN)),
            (vec![4, 0x80, 0, 0, 0, 0, 0, 0, 0], Nbt::Long(i64::MIN)),
            (vec![5, 0x3f, 0x80, 0, 0], Nbt::Float(1.0)),
            (vec![6, 0x3f, 0xf0, 0, 0, 0, 0, 0, 0], Nbt::Double(1.0)),
        ] {
            assert_eq!(Nbt::decode_network(&mut bytes.as_slice()).unwrap(), value);
            assert_eq!(value.encode_network().unwrap(), bytes);
        }
    }

    #[test]
    fn accepts_the_nesting_limit() {
        let mut value = Nbt::Int(1);
        for _ in 0..MAX_DEPTH {
            value = Nbt::List(vec![value]);
        }
        let bytes = value.encode_network().unwrap();
        assert_eq!(Nbt::decode_network(&mut bytes.as_slice()).unwrap(), value);
    }

    #[test]
    fn decodes_fixed_width_array_counts() {
        for (bytes, value) in [
            (
                vec![7, 0, 0, 0, 2, 0x80, 0xff],
                Nbt::ByteArray(vec![0x80, 0xff]),
            ),
            (vec![11, 0, 0, 0, 1, 0, 0, 0, 42], Nbt::IntArray(vec![42])),
            (
                vec![12, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 42],
                Nbt::LongArray(vec![42]),
            ),
        ] {
            assert_eq!(Nbt::decode_network(&mut bytes.as_slice()).unwrap(), value);
            assert_eq!(value.encode_network().unwrap(), bytes);
        }
    }

    #[test]
    fn accepts_empty_lists_with_any_tag_or_negative_count() {
        for bytes in [[9, 0xff, 0, 0, 0, 0], [9, 0, 0xff, 0xff, 0xff, 0xff]] {
            assert_eq!(
                Nbt::decode_network(&mut bytes.as_slice()).unwrap(),
                Nbt::List(Vec::new())
            );
        }
    }

    #[test]
    fn rejects_invalid_tags_and_negative_array_counts() {
        for bytes in [
            vec![0x7f],
            vec![10, 0x7f],
            vec![9, 0, 0, 0, 0, 1],
            vec![9, 13, 0, 0, 0, 1],
            vec![7, 0xff, 0xff, 0xff, 0xff],
            vec![11, 0xff, 0xff, 0xff, 0xff],
            vec![12, 0xff, 0xff, 0xff, 0xff],
        ] {
            assert!(matches!(
                Nbt::decode_network(&mut bytes.as_slice()),
                Err(ProtocolError::InvalidData)
            ));
        }
    }

    #[test]
    fn rejects_excessive_nesting() {
        let mut bytes = vec![10];
        for _ in 0..=MAX_DEPTH {
            bytes.extend_from_slice(&[10, 0, 1, b'a']);
        }
        assert!(matches!(
            Nbt::decode_network(&mut bytes.as_slice()),
            Err(ProtocolError::Overflow)
        ));
    }

    #[test]
    fn rejects_truncated_data_without_allocating_the_declared_array() {
        for bytes in [
            vec![],
            vec![3, 0],
            vec![10, 1, 0, 1, b'a'],
            vec![7, 0x7f, 0xff, 0xff, 0xff],
            vec![11, 0x7f, 0xff, 0xff, 0xff],
            vec![12, 0x7f, 0xff, 0xff, 0xff],
        ] {
            assert!(
                matches!(Nbt::decode_network(&mut bytes.as_slice()), Err(ProtocolError::Io(error))
                if error.kind() == ErrorKind::UnexpectedEof)
            );
        }
    }

    #[test]
    fn consumes_one_root_and_leaves_the_next_value() {
        let mut source = &[7, 0, 0, 0, 1, 0xff, 1, 42][..];
        assert_eq!(
            Nbt::decode_network(&mut source).unwrap(),
            Nbt::ByteArray(vec![0xff])
        );
        assert_eq!(Nbt::decode_network(&mut source).unwrap(), Nbt::Byte(42));
        assert!(source.is_empty());
    }
    #[test]
    fn rejects_invalid_modified_utf8() {
        for payload in [
            vec![0, 1, 0],
            vec![0, 1, 0x80],
            vec![0, 1, 0xc2],
            vec![0, 2, 0xc1, 0x81],
            vec![0, 3, 0xe0, 0x80, 0x80],
            vec![0, 4, 0xf0, 0x9f, 0x98, 0x80],
            vec![0, 3, 0xed, 0xa0, 0xbd],
        ] {
            let mut bytes = vec![8];
            bytes.extend(payload);
            assert!(matches!(
                Nbt::decode_network(&mut bytes.as_slice()),
                Err(ProtocolError::InvalidData)
            ));
        }
    }
}
