use crate::types::base::MCType;
use crate::types::long::MCLong;
use crate::types::ubyte::MCUByte;
use crate::types::varint::MCVarInt;
use std::io::Read;

#[derive(Debug, Clone)]
pub enum PalletedContainer<const INDIRECT_MAX_BITS: u8> {
    SingleValued(MCVarInt),
    Indirect {
        bits_per_entry: MCUByte,
        pallete: Vec<MCVarInt>,
        data: Vec<MCLong>,
    },
    Direct {
        bits_per_entry: MCUByte,
        data: Vec<MCLong>,
    },
}

impl<const INDIRECT_MAX_BITS: u8> MCType for PalletedContainer<INDIRECT_MAX_BITS> {
    fn pack(&self) -> Vec<u8> {
        match self {
            PalletedContainer::SingleValued(block_id) => {
                let mut result = Vec::new();
                result.extend(MCUByte::from(0).pack()); // bits_per_entry
                result.extend(block_id.pack());
                result.extend(MCVarInt::from(0).pack()); // data array len
                result
            }
            PalletedContainer::Indirect {
                bits_per_entry,
                pallete,
                data,
            } => {
                let mut result = Vec::new();
                result.extend(bits_per_entry.pack());
                result.extend(pallete.pack());
                result.extend(data.pack());
                result
            }
            PalletedContainer::Direct {
                bits_per_entry,
                data,
            } => {
                let mut result = Vec::new();
                result.extend(bits_per_entry.pack());
                result.extend(data.pack());
                result
            }
        }
    }

    fn unpack(src: &mut dyn Read) -> Self {
        // TODO for block and biome
        let bits_per_entry = MCUByte::unpack(src);
        if bits_per_entry == 0 {
            let value = MCVarInt::unpack(src);
            let _data_len = MCVarInt::unpack(src);
            Self::SingleValued(value)
        } else if <MCUByte as Into<u8>>::into(bits_per_entry.clone()) <= INDIRECT_MAX_BITS {
            let pallete_length = MCVarInt::unpack(src).into();
            let mut pallete = Vec::new();
            for _ in 0..pallete_length {
                pallete.push(MCVarInt::unpack(src));
            }
            let mut data = Vec::new();
            let data_length = MCVarInt::unpack(src).into();
            for _ in 0..data_length {
                data.push(MCLong::unpack(src));
            }
            Self::Indirect {
                bits_per_entry,
                pallete,
                data,
            }
        } else {
            let mut data = Vec::new();
            let data_length = MCVarInt::unpack(src).into();
            for _ in 0..data_length {
                data.push(MCLong::unpack(src));
            }
            Self::Direct {
                bits_per_entry,
                data,
            }
        }
    }
}
