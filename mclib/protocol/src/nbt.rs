use crate::nbt::tags::base::{unpack_by_ty_id, NBTTag};
use crate::utils::TcpUtils;
use std::io::Read;

pub mod tags;

#[derive(Debug)]
pub struct NBT(pub Option<String>, pub Box<dyn NBTTag>);

impl Clone for NBT {
    // TODO
    fn clone(&self) -> Self {
        let bin = self.pack();
        Self::unpack(&mut std::io::Cursor::new(bin), self.0.is_some())
    }
}

impl NBT {
    pub fn pack(&self) -> Vec<u8> {
        let mut result = Vec::new();
        result.push(self.1.ty_id());
        if let Some(name) = &self.0 {
            let name = name.as_bytes();
            result.extend((name.len() as u16).to_be_bytes());
            result.extend(name);
        }
        result.extend(self.1.pack());
        result
    }

    pub fn unpack(src: &mut dyn Read, read_name: bool) -> Self {
        let ty_id = src.read_byte();
        let name = if read_name {
            let name_length = i16::from_be_bytes([src.read_byte(), src.read_byte()]);
            let mut name = Vec::new();
            for _ in 0..name_length {
                name.push(src.read_byte());
            }
            Some(String::from_utf8(name).unwrap())
        } else {
            None
        };
        let inner = unpack_by_ty_id(ty_id, src);
        Self(name, inner)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::nbt::tags::base::IntoNBTTag;
    use std::io::Read;

    #[test]
    fn test_big_test_gzip() {
        let result: Vec<u8> = vec![
            31, 139, 8, 0, 0, 0, 0, 0, 0, 0, 237, 84, 207, 79, 26, 65, 20, 126, 194, 2, 203, 150,
            130, 177, 196, 16, 99, 204, 171, 181, 132, 165, 219, 205, 66, 17, 137, 177, 136, 22,
            44, 154, 13, 26, 216, 168, 49, 134, 184, 43, 195, 130, 46, 187, 102, 119, 176, 241,
            212, 75, 123, 108, 122, 235, 63, 211, 35, 127, 67, 207, 189, 246, 191, 160, 195, 47,
            123, 105, 207, 189, 240, 50, 201, 247, 230, 189, 111, 230, 123, 111, 38, 121, 2, 4, 84,
            114, 79, 44, 14, 120, 203, 177, 77, 141, 120, 244, 227, 112, 98, 62, 8, 123, 29, 199,
            165, 147, 24, 15, 130, 71, 221, 238, 132, 2, 98, 181, 162, 170, 199, 120, 118, 92, 87,
            203, 168, 85, 15, 27, 200, 214, 30, 106, 149, 134, 134, 13, 173, 126, 88, 123, 143,
            131, 207, 131, 79, 131, 111, 207, 3, 16, 110, 91, 142, 62, 190, 165, 56, 76, 100, 253,
            16, 234, 218, 116, 166, 35, 64, 220, 102, 46, 105, 225, 181, 211, 187, 115, 250, 118,
            11, 41, 219, 11, 224, 239, 232, 61, 30, 56, 91, 239, 17, 8, 86, 245, 222, 93, 223, 11,
            64, 224, 94, 183, 250, 100, 183, 4, 0, 140, 65, 76, 115, 198, 8, 85, 76, 211, 32, 46,
            125, 164, 192, 200, 194, 16, 179, 186, 222, 88, 11, 83, 163, 238, 68, 142, 69, 3, 48,
            177, 39, 83, 140, 76, 241, 233, 20, 163, 83, 140, 133, 225, 217, 159, 227, 179, 242,
            68, 129, 165, 124, 51, 221, 216, 187, 199, 170, 117, 19, 95, 40, 28, 8, 215, 46, 209,
            89, 63, 175, 29, 27, 96, 33, 89, 223, 250, 241, 5, 254, 193, 206, 252, 157, 189, 0,
            188, 241, 64, 201, 248, 133, 66, 64, 70, 254, 158, 235, 234, 15, 147, 58, 104, 135, 96,
            187, 235, 50, 55, 163, 40, 10, 142, 187, 245, 208, 105, 99, 202, 78, 219, 233, 236,
            230, 230, 43, 59, 189, 37, 190, 100, 73, 9, 61, 170, 187, 148, 253, 24, 126, 232, 210,
            14, 218, 111, 21, 76, 177, 104, 62, 43, 225, 155, 156, 132, 153, 188, 132, 5, 9, 101,
            89, 22, 69, 0, 255, 47, 40, 174, 47, 242, 194, 178, 164, 46, 29, 32, 119, 90, 59, 185,
            140, 202, 231, 41, 223, 81, 65, 201, 22, 181, 197, 109, 161, 42, 173, 44, 197, 49, 127,
            186, 122, 146, 142, 94, 157, 95, 248, 18, 5, 35, 27, 209, 246, 183, 119, 170, 205, 149,
            114, 188, 158, 223, 88, 93, 75, 151, 174, 146, 23, 185, 68, 208, 128, 200, 250, 62,
            191, 179, 220, 84, 203, 7, 117, 110, 163, 182, 118, 89, 146, 147, 169, 220, 81, 80,
            153, 107, 204, 53, 230, 26, 255, 87, 35, 8, 66, 203, 233, 27, 214, 120, 194, 236, 254,
            252, 122, 251, 125, 120, 211, 132, 223, 212, 242, 164, 251, 8, 6, 0, 0,
        ];

        let nbt = NBT(
            Some("Level".to_string()),
            vec![
                ("longTest", 9223372036854775807i64.to_nbt()),
                ("shortTest", 32767i16.to_nbt()),
                (
                    "stringTest",
                    "HELLO WORLD THIS IS A TEST STRING \u{c5}\u{c4}\u{d6}!".to_nbt(),
                ),
                ("floatTest", 0.49823147058486938f32.to_nbt()),
                ("intTest", 2147483647i32.to_nbt()),
                (
                    "nested compound test",
                    vec![
                        (
                            "ham",
                            vec![("name", "Hampus".to_nbt()), ("value", 0.75f32.to_nbt())].to_nbt(),
                        ),
                        (
                            "egg",
                            vec![("name", "Eggbert".to_nbt()), ("value", 0.5f32.to_nbt())].to_nbt(),
                        ),
                    ]
                    .to_nbt(),
                ),
                (
                    "listTest (long)",
                    vec![
                        11i64.to_nbt(),
                        12i64.to_nbt(),
                        13i64.to_nbt(),
                        14i64.to_nbt(),
                        15i64.to_nbt(),
                    ]
                    .to_nbt(),
                ),
                (
                    "listTest (compound)",
                    vec![
                        vec![
                            ("name", "Compound tag #0".to_nbt()),
                            ("created-on", 1264099775885i64.to_nbt()),
                        ]
                        .to_nbt(),
                        vec![
                            ("name", "Compound tag #1".to_nbt()),
                            ("created-on", 1264099775885i64.to_nbt()),
                        ]
                        .to_nbt(),
                    ]
                    .to_nbt(),
                ),
                ("byteTest", 127i8.to_nbt()),
                (
                    "byteArrayTest (the first 1000 values of (n*n*255+n*7)%100, starting with n=0 (0, 62, 34, 16, 8, ...))",
                    {
                        let mut result = Vec::new();
                        for n in 0..1000 {
                            let value = (n * n * 255 + n * 7) % 100;
                            result.push(value as i8);
                        }
                        result
                    }
                    .to_nbt(),
                ),
                ("doubleTest", 0.49312871321823148f64.to_nbt())
            ]
            .to_nbt(),
        );

        let mut decoder = flate2::read::GzDecoder::new(std::io::Cursor::new(result));
        let mut result = Vec::new();
        decoder.read_to_end(&mut result).unwrap();
        assert_eq!(nbt.pack(), result);

        assert_eq!(
            NBT::unpack(&mut std::io::Cursor::new(result.clone()), true).pack(),
            result
        )
    }

    #[test]
    fn test_hello_world() {
        let result = vec![
            0x0a, 0x00, 0x0b, 0x68, 0x65, 0x6c, 0x6c, 0x6f, 0x20, 0x77, 0x6f, 0x72, 0x6c, 0x64,
            0x08, 0x00, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x00, 0x09, 0x42, 0x61, 0x6e, 0x61, 0x6e,
            0x72, 0x61, 0x6d, 0x61, 0x00,
        ];

        let compound = NBT(
            Some("hello world".to_string()),
            vec![("name", "Bananrama".to_nbt())].to_nbt(),
        );

        assert_eq!(compound.pack(), result);
    }
}
