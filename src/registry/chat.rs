use mclib::nbt::{IntoNBTTag, NBTTag};

pub struct Decoration {
    /// The translation key representing the chat format. It can also be a formatting string directly.
    ///
    /// Example: `chat.type.text`, which translates to `<%s> %s`.
    pub translation_key: String,
    // /// Optional styling to be applied on the final message.
    // /// Not present in the narration decoration.
    // pub style: Option<_>
    /// Placeholders used when formatting the string given by the translation_key field.
    ///
    /// Can be either:
    /// * `sender`, for the name of the player sending the message.
    /// * `target`, for the name of the player receiving the message, which may be empty.
    /// * `content`, for the actual message.
    pub parameters: String,
}

impl IntoNBTTag for Decoration {
    fn to_nbt(self) -> Box<dyn NBTTag> {
        vec![
            ("translation_key", self.translation_key.to_nbt()),
            ("parameters", self.parameters.to_nbt()),
        ]
        .to_nbt()
    }
}

/// The `minecraft:chat_type` registry. It defines the different types of in-game chat and how they're formatted.
pub struct Chat {
    /// The chat decoration.
    pub chat: Decoration,
    /// The narration decoration.
    pub narration: Decoration,
}

impl IntoNBTTag for Chat {
    fn to_nbt(self) -> Box<dyn NBTTag> {
        vec![
            ("chat", self.chat.to_nbt()),
            ("narration", self.narration.to_nbt()),
        ]
        .to_nbt()
    }
}
