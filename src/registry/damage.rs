use mclib::nbt::{IntoNBTTag, NBTTag};

pub struct DamageType {
    /// Id of the death message. The full message is displayed as `death.attack.<message_id>`.
    ///
    /// Example: "onFire".
    pub message_id: String,
    /// Whether the damage taken scales with the difficulty.
    ///
    /// Can be either:
    /// * `never`
    /// * `when_caused_by_living_non_player`
    /// * `always`
    pub scaling: String,
    /// The amount of exhaustion caused when suffering this type of damage.
    /// Default values are either 0.0 or 0.1.
    pub exhaustion: f32,
    // /// Effect played when the player suffers this damage, including the sound that is played.
    // ///
    // /// Can be either:
    // /// * `hurt`
    // /// * `thorns`
    // /// * `drowning`
    // /// * `burning`
    // /// * `poking`
    // /// * `freezing`
    // pub effects: Option<String>,
    // /// Defines how the death message is constructed.
    // ///
    // /// Can be either:
    // /// * `default`, for the message to be built normally.
    // /// * `fall_variants`, for the most significant fall damage to be considered.
    // /// * `intentional_game_design`, for MCPE-28723 to be considered as an argument when translating the message.
    // pub death_message_type: Option<String>,
}

impl IntoNBTTag for DamageType {
    fn to_nbt(self) -> Box<dyn NBTTag> {
        vec![
            ("message_id", self.message_id.to_nbt()),
            ("scaling", self.scaling.to_nbt()),
            ("exhaustion", self.exhaustion.to_nbt()),
        ]
        .to_nbt()
    }
}
