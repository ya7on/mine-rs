pub enum NBT {
    /// Root Compound
    Root((String, Vec<(String, NBT)>)),
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    ByteArray(Vec<u8>),
    String(String),
    List(Vec<NBT>),
    Compound(Vec<(String, NBT)>),
    IntArray(Vec<i32>),
    LongArray(Vec<i64>),
}

impl NBT {
    pub fn to_hex(&self) -> Vec<u8> {
        match self {
            NBT::Root((root_name, childs)) => {
                let mut result = Vec::new();
                result.push(self.type_id());
                let bytes_name = root_name.as_bytes();
                result.extend_from_slice(&(bytes_name.len() as u16).to_be_bytes());
                result.extend_from_slice(bytes_name);

                for (child_name, child_tag) in childs.iter() {
                    result.push(child_tag.type_id());
                    let bytes_child_name = child_name.as_bytes();
                    result.extend_from_slice(&(bytes_child_name.len() as u16).to_be_bytes());
                    result.extend_from_slice(bytes_child_name);
                    result.extend_from_slice(&child_tag.to_hex());
                }

                result.push(0);

                result
            }
            NBT::Byte(value) => value.to_be_bytes().to_vec(),
            NBT::Short(value) => value.to_be_bytes().to_vec(),
            NBT::Int(value) => value.to_be_bytes().to_vec(),
            NBT::Long(value) => value.to_be_bytes().to_vec(),
            NBT::Float(value) => value.to_be_bytes().to_vec(),
            NBT::Double(value) => value.to_be_bytes().to_vec(),
            NBT::ByteArray(value) => {
                let mut result = Vec::new();
                result.extend_from_slice(&(value.len() as i32).to_be_bytes());
                result.extend_from_slice(value);
                result
            }
            NBT::String(value) => {
                let mut result = Vec::new();

                let bytes_value = value.as_bytes();
                result.extend_from_slice(&(bytes_value.len() as u16).to_be_bytes());
                result.extend_from_slice(bytes_value);

                result
            }
            NBT::List(items) => {
                let mut result = Vec::new();
                if let Some(first) = items.first() {
                    result.push(first.type_id());
                    result.extend_from_slice(&(items.len() as i32).to_be_bytes());
                    for item in items {
                        result.extend_from_slice(&item.to_hex());
                    }
                } else {
                    result.push(0);
                }
                result
            }
            NBT::Compound(childs) => {
                let mut result = Vec::new();

                for (child_name, child_tag) in childs.iter() {
                    result.push(child_tag.type_id());
                    let bytes_child_name = child_name.as_bytes();
                    result.extend_from_slice(&(bytes_child_name.len() as u16).to_be_bytes());
                    result.extend_from_slice(bytes_child_name);
                    result.extend_from_slice(&child_tag.to_hex());
                }

                result.push(0);

                result
            }
            NBT::IntArray(values) => {
                let mut result = Vec::new();
                result.extend_from_slice(&(values.len() as i32).to_be_bytes());
                for item in values {
                    result.extend_from_slice(&item.to_be_bytes());
                }
                result
            }
            NBT::LongArray(values) => {
                let mut result = Vec::new();
                result.extend_from_slice(&(values.len() as i32).to_be_bytes());
                for item in values {
                    result.extend_from_slice(&item.to_be_bytes());
                }
                result
            }
        }
    }

    pub fn type_id(&self) -> u8 {
        match self {
            NBT::Root(_) => 10,
            NBT::Byte(_) => 1,
            NBT::Short(_) => 2,
            NBT::Int(_) => 3,
            NBT::Long(_) => 4,
            NBT::Float(_) => 5,
            NBT::Double(_) => 6,
            NBT::ByteArray(_) => 7,
            NBT::String(_) => 8,
            NBT::List(_) => 9,
            NBT::Compound(_) => 10,
            NBT::IntArray(_) => 11,
            NBT::LongArray(_) => 12,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nbt_create() {
        let nbt = NBT::Root((
            "hello world".to_string(),
            vec![("name".to_string(), NBT::String("Bananrama".to_string()))],
        ));
        assert_eq!(
            nbt.to_hex(),
            vec![
                0x0a, // root tag type id - Compound
                0x00, 0x0b, // length of root tag name - 11
                0x68, 0x65, 0x6c, 0x6c, 0x6f, 0x20, 0x77, 0x6f, 0x72, 0x6c,
                0x64, // root tag name - hello world
                0x08, // string type id
                0x00, 0x04, // string name length - 4
                0x6e, 0x61, 0x6d, 0x65, // string name - name
                0x00, 0x09, // length of string value - 9
                0x42, 0x61, 0x6e, 0x61, 0x6e, 0x72, 0x61, 0x6d,
                0x61, // value of string - Bananrama
                0x00, // tag end
            ]
        );
    }
}
