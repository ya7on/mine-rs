use crate::registry::Description;
use mclib::nbt::{IntoNBTTag, NBTTag};
use serde::Deserialize;

/// The minecraft:trim_material registry. It defines various visual properties of trim materials in armors.
#[derive(Deserialize, Debug)]
pub struct ArmorTrimMaterial {
    /// The trim color model to be rendered on top of the armor.
    /// The Notchian client uses the corresponding asset located at trims/color_palettes.
    ///
    /// Example: `minecraft:amethyst`.
    pub asset_name: String,
    /// The ingredient used.
    /// This has the visual effect of showing the trimmed armor model on the Smithing Table when the correct item is placed.
    ///
    /// Example: `minecraft:copper_ingot`.
    pub ingredient: String,
    /// Color index of the trim on the armor item when in the inventory.
    /// Default values vary between 0.1 and 1.0.
    pub item_model_index: f32,
    // TODO
    // /// Asset for different types of armor materials, which overrides the value specified in the asset_name field.
    // /// The Notchian client uses this to give a darker color shade when a trim material is applied to armor of the same material, such as iron applied to iron armor.
    // pub override_armor_materials: Option<_>
    /// The name of the trim material to be displayed on the armor tool-tip.
    /// Any styling used in this component is also applied to the trim pattern description.
    pub description: Description,
}

impl IntoNBTTag for ArmorTrimMaterial {
    fn to_nbt(self) -> Box<dyn NBTTag> {
        vec![
            ("asset_name", self.asset_name.to_nbt()),
            ("ingredient", self.ingredient.to_nbt()),
            ("item_model_index", self.item_model_index.to_nbt()),
            ("description", self.description.to_nbt()),
        ]
        .to_nbt()
    }
}
