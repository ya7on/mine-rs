use crate::registry::entry::RegistryEntry;
use mclib::nbt::NBT;
use mclib::nbt::{IntoNBTTag, NBTTag};
use serde::de::Unexpected;
use serde::{de, Deserialize, Deserializer};

pub mod biome;
pub mod chat;
pub mod damage;
pub mod dimension;
pub mod entry;
pub mod trim_material;
pub mod trim_pattern;

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum IntegerDistributionValue {
    Uniform {
        min_inclusive: i32,
        max_inclusive: i32,
    },
}

impl IntoNBTTag for IntegerDistributionValue {
    fn to_nbt(self) -> Box<dyn NBTTag> {
        match self {
            IntegerDistributionValue::Uniform {
                min_inclusive,
                max_inclusive,
            } => vec![
                ("min_inclusive", min_inclusive.to_nbt()),
                ("max_inclusive", max_inclusive.to_nbt()),
            ]
            .to_nbt(),
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct Description {
    // TODO color
    pub translate: String,
}

impl IntoNBTTag for Description {
    fn to_nbt(self) -> Box<dyn NBTTag> {
        vec![("translate", self.translate.to_nbt())].to_nbt()
    }
}

fn bool_from_int<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    match u8::deserialize(deserializer)? {
        0 => Ok(false),
        1 => Ok(true),
        other => Err(de::Error::invalid_value(
            Unexpected::Unsigned(other as u64),
            &"zero or one",
        )),
    }
}

#[derive(Deserialize, Debug)]
pub struct Registry<T: IntoNBTTag> {
    #[serde(rename = "type")]
    pub ty: String,
    pub value: Vec<RegistryEntry<T>>,
}

#[derive(Deserialize, Debug)]
pub struct RegistryData {
    #[serde(rename = "minecraft:trim_pattern")]
    trim_patterns: Registry<trim_pattern::ArmorTrimPattern>,
    #[serde(rename = "minecraft:trim_material")]
    trim_materials: Registry<trim_material::ArmorTrimMaterial>,
    #[serde(rename = "minecraft:chat_type")]
    chat_types: Registry<chat::Chat>,
    #[serde(rename = "minecraft:dimension_type")]
    dimension_types: Registry<dimension::DimensionType>,
    #[serde(rename = "minecraft:damage_type")]
    damage_types: Registry<damage::DamageType>,
    #[serde(rename = "minecraft:worldgen/biome")]
    biomes: Registry<biome::Biome>,
}

impl From<RegistryData> for NBT {
    fn from(value: RegistryData) -> Self {
        let mut trim_patterns = Vec::<Box<dyn NBTTag>>::new();
        for trim_pattern in value.trim_patterns.value {
            trim_patterns.push(trim_pattern.to_nbt())
        }

        let mut trim_materials = Vec::<Box<dyn NBTTag>>::new();
        for trim_material in value.trim_materials.value {
            trim_materials.push(trim_material.to_nbt())
        }

        let mut chat_types = Vec::<Box<dyn NBTTag>>::new();
        for chat_type in value.chat_types.value {
            chat_types.push(chat_type.to_nbt())
        }

        let mut dimension_types = Vec::new();
        for dimension_type in value.dimension_types.value {
            dimension_types.push(dimension_type.to_nbt());
        }

        let mut damage_types = Vec::new();
        for damage_type in value.damage_types.value {
            damage_types.push(damage_type.to_nbt());
        }

        let mut biomes = Vec::<Box<dyn NBTTag>>::new();
        for biome in value.biomes.value {
            biomes.push(biome.to_nbt())
        }

        NBT(
            None,
            vec![
                (
                    "minecraft:trim_pattern",
                    vec![
                        ("type", "minecraft:trim_pattern".to_nbt()),
                        ("value", trim_patterns.to_nbt()),
                    ]
                    .to_nbt(),
                ),
                (
                    "minecraft:trim_material",
                    vec![
                        ("type", "minecraft:trim_material".to_nbt()),
                        ("value", trim_materials.to_nbt()),
                    ]
                    .to_nbt(),
                ),
                (
                    "minecraft:chat_type",
                    vec![
                        ("type", "minecraft:chat_type".to_nbt()),
                        ("value", chat_types.to_nbt()),
                    ]
                    .to_nbt(),
                ),
                (
                    "minecraft:dimension_type",
                    vec![
                        ("type", "minecraft:dimension_type".to_nbt()),
                        ("value", dimension_types.to_nbt()),
                    ]
                    .to_nbt(),
                ),
                (
                    "minecraft:damage_type",
                    vec![
                        ("type", "minecraft:damage_type".to_nbt()),
                        ("value", damage_types.to_nbt()),
                    ]
                    .to_nbt(),
                ),
                (
                    "minecraft:worldgen/biome",
                    vec![
                        ("type", "minecraft:worldgen/biome".to_nbt()),
                        ("value", biomes.to_nbt()),
                    ]
                    .to_nbt(),
                ),
            ]
            .to_nbt(),
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::registry::RegistryData;

    #[test]
    fn test_serde() {
        let reference = include_str!("../assets/registry_data_1.20.2.json");

        let r = serde_json::from_str::<RegistryData>(reference);
        assert!(r.is_ok(), "{:?}", r)
    }
}
