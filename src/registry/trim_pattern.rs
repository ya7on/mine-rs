use crate::registry::{bool_from_int, Description};
use mclib::nbt::{IntoNBTTag, NBTTag};
use serde::Deserialize;

/// The minecraft:trim_pattern registry. It defines various visual properties of trim patterns in armors.
#[derive(Deserialize, Debug)]
pub struct ArmorTrimPattern {
    /// The trim pattern model to be rendered on top of the armor.
    /// The Notchian client uses the corresponding asset located at trims/models/armor.
    ///
    /// Example: `minecraft:coast`.
    pub asset_id: String,
    /// The template item used for this trim.
    /// This has the visual effect of showing the trimmed armor model on the Smithing Table when the correct item is placed.
    ///
    /// Example: `minecraft:coast_armor_trim_smithing_template`.
    pub template_item: String,
    /// The name of the trim pattern to be displayed on the armor tool-tip.
    pub description: Description,
    /// Whether this trim is a decal.
    #[serde(deserialize_with = "bool_from_int")]
    pub decal: bool,
}

impl IntoNBTTag for ArmorTrimPattern {
    fn to_nbt(self) -> Box<dyn NBTTag> {
        vec![
            ("asset_id", self.asset_id.to_nbt()),
            ("template_item", self.template_item.to_nbt()),
            ("description", self.description.to_nbt()),
            ("decal", (self.decal as i8).to_nbt()),
        ]
        .to_nbt()
    }
}
