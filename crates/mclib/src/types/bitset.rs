use std::io::Read;

use crate::{MCByteArray, MCType, ProtocolError};

/// Protocol-777 `BitSet`: a VarInt-prefixed byte array, lowest bits first.
///
/// The in-memory representation retains 64-bit words; wire bytes follow
/// Java's `BitSet.toByteArray()` rather than the older long-array encoding.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct MCBitSet(pub Vec<u64>);

/// Maximum number of words accepted when decoding a bit set.
const MAX_WORDS: usize = 8_192;
const MAX_BYTES: usize = MAX_WORDS * 8;

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
        if self.0.len() > MAX_WORDS {
            return Err(ProtocolError::Overflow);
        }
        let mut bytes = Vec::with_capacity(self.0.len() * 8);
        for word in &self.0 {
            bytes.extend_from_slice(&word.to_le_bytes());
        }
        // Java omits trailing zero bytes, including entire unused words.
        while bytes.last() == Some(&0) {
            bytes.pop();
        }
        MCByteArray::<MAX_BYTES>(bytes).pack()
    }

    fn unpack(source: &mut dyn Read) -> Result<Self, ProtocolError> {
        let bytes = MCByteArray::<MAX_BYTES>::unpack(source)?.0;
        let mut words = Vec::with_capacity(bytes.len().div_ceil(8));
        for chunk in bytes.chunks(8) {
            let mut word = [0_u8; 8];
            word[..chunk.len()].copy_from_slice(chunk);
            words.push(u64::from_le_bytes(word));
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

        assert_eq!(encoded[0], 17);
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
        let encoded = crate::MCVarInt(100_000).pack().unwrap();

        let error = MCBitSet::unpack(&mut Cursor::new(encoded)).unwrap_err();

        assert!(matches!(error, ProtocolError::Overflow));
    }

    #[test]
    fn matches_vanilla_26_layer_light_mask() {
        let mask = MCBitSet(vec![(1 << 26) - 1]);
        // ByteBufCodecs.BIT_SET uses BitSet.toByteArray(): little endian bytes.
        let encoded = [4, 255, 255, 255, 3];
        assert_eq!(mask.pack().unwrap(), encoded);
        let mut input = &[4, 255, 255, 255, 3, 26][..];
        assert_eq!(MCBitSet::unpack(&mut input).unwrap(), mask);
        assert_eq!(input, [26]);
        assert_eq!(MCBitSet(vec![0, 0]).pack().unwrap(), [0]);
    }
}
