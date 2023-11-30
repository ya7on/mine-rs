use crate::types::base::MCType;
use crate::types::varint::MCVarInt;
use crate::utils::TcpUtils;
use std::io::Read;

// TODO max length?
#[derive(Debug, Clone)]
pub struct MCString(String);

impl From<String> for MCString {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for MCString {
    fn from(value: &str) -> Self {
        Self(value.to_owned())
    }
}

impl From<MCString> for String {
    fn from(value: MCString) -> Self {
        value.0
    }
}

impl PartialEq<String> for MCString {
    fn eq(&self, other: &String) -> bool {
        &self.0 == other
    }
}

impl MCType for MCString {
    fn pack(&self) -> Vec<u8> {
        let mut result = Vec::new();
        result.extend(MCVarInt::from(self.0.len() as i32).pack());
        result.extend(self.0.bytes());
        result
    }

    fn unpack(src: &mut dyn Read) -> Self {
        let string_length = MCVarInt::unpack(src);
        let mut string_buffer = Vec::new();

        for _ in 0..string_length.into() {
            string_buffer.push(src.read_byte());
        }

        let string_result = String::from_utf8(string_buffer).unwrap_or_default(); // TODO add error handling

        Self(string_result)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    #[ignore]
    fn test_string_pack() {
        unimplemented!()
    }

    #[test]
    #[ignore]
    fn test_string_unpack() {
        unimplemented!()
    }
}
