use std::io::Read;

use crate::{MCType, MCVarInt, ProtocolError};

/// A length-prefixed Minecraft `BitSet`, packed into 64-bit words.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct MCBitSet(pub Vec<u64>);

/// Maximum number of words accepted when decoding a bit set.
const MAX_WORDS: usize = 8_192;

impl MCBitSet {
    /// Sets the bit at `index`, growing the backing storage as needed.
    pub fn set(&mut self, index: usize) {
        let word = index / 64;
        if word >= self.0.len() {
            self.0.resize(word + 1, 0);
        }
        self.0[word] |= 1 << (index % 64);
    }

    #[must_use]
    pub fn get(&self, index: usize) -> bool {
        self.0
            .get(index / 64)
            .is_some_and(|word| word & (1 << (index % 64)) != 0)
    }
}

impl MCType for MCBitSet {
    fn pack(&self) -> Result<Vec<u8>, ProtocolError> {
        let length = i32::try_from(self.0.len()).map_err(|_| ProtocolError::Overflow)?;
        let mut bytes = MCVarInt(length).pack()?;
        for word in &self.0 {
            bytes.extend_from_slice(&word.to_be_bytes());
        }
        Ok(bytes)
    }

    fn unpack(source: &mut dyn Read) -> Result<Self, ProtocolError> {
        let length = MCVarInt::unpack(source)?.0;
        let length = usize::try_from(length).map_err(|_| ProtocolError::InvalidData)?;
        // A bit set is bounded by what a payload can realistically carry; a
        // declared length of millions of words would exhaust memory before any
        // payload could supply them.
        if length > MAX_WORDS {
            return Err(ProtocolError::Overflow);
        }

        let mut words = Vec::with_capacity(length);
        for _ in 0..length {
            let mut bytes = [0_u8; 8];
            source.read_exact(&mut bytes)?;
            words.push(u64::from_be_bytes(bytes));
        }
        Ok(Self(words))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn roundtrips() {
        let mut value = MCBitSet::default();
        value.set(0);
        value.set(64);
        value.set(129);

        let encoded = value.pack().unwrap();

        assert_eq!(encoded[0], 0x03);
        assert_eq!(MCBitSet::unpack(&mut Cursor::new(encoded)).unwrap(), value);
        assert!(value.get(0) && value.get(64) && value.get(129));
        assert!(!value.get(1) && !value.get(65) && !value.get(128));
    }

    #[test]
    fn empty_bit_set_packs_as_zero() {
        let encoded = MCBitSet::default().pack().unwrap();

        assert_eq!(encoded, [0x00]);
    }

    #[test]
    fn rejects_huge_declared_lengths() {
        let encoded = MCVarInt(100_000).pack().unwrap();

        let error = MCBitSet::unpack(&mut Cursor::new(encoded)).unwrap_err();

        assert!(matches!(error, ProtocolError::Overflow));
    }
}
