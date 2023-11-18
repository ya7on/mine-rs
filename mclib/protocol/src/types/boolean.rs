use crate::types::base::MCType;

const MC_BOOLEAN_TRUE: u8 = 0x01;

#[derive(Debug)]
pub struct MCBoolean(bool);

impl From<bool> for MCBoolean {
    fn from(value: bool) -> Self {
        Self(value)
    }
}

impl Into<bool> for MCBoolean {
    fn into(self) -> bool {
        self.0
    }
}

impl PartialEq<bool> for MCBoolean {
    fn eq(&self, other: &bool) -> bool {
        &self.0 == other
    }
}

impl MCType for MCBoolean {
    fn pack(&self) -> Vec<u8> {
        vec![self.0 as u8]
    }

    fn unpack(src: &mut Vec<u8>) -> Self {
        Self(Self::read_byte(src) == MC_BOOLEAN_TRUE)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_boolean_pack() {
        assert_eq!(vec![0b1], MCBoolean::from(true).pack());
        assert_eq!(vec![0b0], MCBoolean::from(false).pack());
    }

    #[test]
    fn test_boolean_unpack() {
        assert_eq!(MCBoolean::unpack(&mut vec![0b1]), true);
        assert_eq!(MCBoolean::unpack(&mut vec![0b0]), false);
    }
}