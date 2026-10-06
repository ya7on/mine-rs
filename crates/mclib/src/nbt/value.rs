//! The NBT value tree.

/// Maximum nesting depth accepted by the network codecs.
pub(super) const MAX_DEPTH: usize = 512;

/// An NBT value.
#[derive(Debug, Clone, PartialEq)]
pub enum Nbt {
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    ByteArray(Vec<u8>),
    String(String),
    List(Vec<Self>),
    Compound(Vec<(String, Self)>),
    IntArray(Vec<i32>),
    LongArray(Vec<i64>),
}

impl Nbt {
    /// Builds a compound from name-value pairs.
    #[must_use]
    pub fn compound(entries: Vec<(&str, Self)>) -> Self {
        Self::Compound(
            entries
                .into_iter()
                .map(|(name, value)| (name.to_owned(), value))
                .collect(),
        )
    }

    pub fn string(value: impl Into<String>) -> Self {
        Self::String(value.into())
    }
}

/// Returns the wire tag byte that encodes this value's variant.
pub(super) const fn tag_of(value: &Nbt) -> u8 {
    match value {
        Nbt::Byte(_) => 1,
        Nbt::Short(_) => 2,
        Nbt::Int(_) => 3,
        Nbt::Long(_) => 4,
        Nbt::Float(_) => 5,
        Nbt::Double(_) => 6,
        Nbt::ByteArray(_) => 7,
        Nbt::String(_) => 8,
        Nbt::List(_) => 9,
        Nbt::Compound(_) => 10,
        Nbt::IntArray(_) => 11,
        Nbt::LongArray(_) => 12,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_a_compound_from_pairs() {
        let value = Nbt::compound(vec![("name", Nbt::string("minecraft:overworld"))]);

        assert_eq!(
            value,
            Nbt::Compound(vec![(
                "name".to_owned(),
                Nbt::String("minecraft:overworld".to_owned())
            )])
        );
    }
}
