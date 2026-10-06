use std::io::Read;

use crate::{MCBoolean, MCByte, MCString, MCType, MCUnsignedByte, MCVarInt, ProtocolError};

/// Client settings sent during Configuration (protocol 777).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClientInformation {
    pub locale: MCString<16>,
    pub view_distance: MCByte,
    pub chat_mode: MCVarInt,
    pub chat_colors: MCBoolean,
    pub skin_parts: MCUnsignedByte,
    pub main_hand: MCVarInt,
    pub text_filtering: MCBoolean,
    pub server_listings: MCBoolean,
    pub particle_status: MCVarInt,
}

impl MCType for ClientInformation {
    fn pack(&self) -> Result<Vec<u8>, ProtocolError> {
        let mut bytes = self.locale.pack()?;
        bytes.extend(self.view_distance.pack()?);
        bytes.extend(self.chat_mode.pack()?);
        bytes.extend(self.chat_colors.pack()?);
        bytes.extend(self.skin_parts.pack()?);
        bytes.extend(self.main_hand.pack()?);
        bytes.extend(self.text_filtering.pack()?);
        bytes.extend(self.server_listings.pack()?);
        bytes.extend(self.particle_status.pack()?);
        Ok(bytes)
    }

    fn unpack(source: &mut dyn Read) -> Result<Self, ProtocolError> {
        let value = Self {
            locale: MCString::unpack(source)?,
            view_distance: MCByte::unpack(source)?,
            chat_mode: MCVarInt::unpack(source)?,
            chat_colors: MCBoolean::unpack(source)?,
            skin_parts: MCUnsignedByte::unpack(source)?,
            main_hand: MCVarInt::unpack(source)?,
            text_filtering: MCBoolean::unpack(source)?,
            server_listings: MCBoolean::unpack(source)?,
            particle_status: MCVarInt::unpack(source)?,
        };
        if !(0..=2).contains(&value.chat_mode.0)
            || !(0..=1).contains(&value.main_hand.0)
            || !(0..=2).contains(&value.particle_status.0)
        {
            return Err(ProtocolError::InvalidData);
        }
        Ok(value)
    }
}
