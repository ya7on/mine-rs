use crate::registry::entry::RegistryEntry;
use mclib::nbt::NBT;
use mclib::nbt::{IntoNBTTag, NBTTag};

pub mod biome;
pub mod chat;
pub mod damage;
pub mod dimension;
pub mod entry;
pub mod trim_material;
pub mod trim_pattern;

pub struct RegistryData {
    trim_patterns: Vec<RegistryEntry<trim_pattern::ArmorTrimPattern>>,
    trim_materials: Vec<RegistryEntry<trim_material::ArmorTrimMaterial>>,
    chat_types: Vec<RegistryEntry<chat::Chat>>,
    dimension_types: Vec<RegistryEntry<dimension::DimensionType>>,
    damage_types: Vec<RegistryEntry<damage::DamageType>>,
    biomes: Vec<RegistryEntry<biome::Biome>>,
}

impl From<RegistryData> for NBT {
    fn from(value: RegistryData) -> Self {
        let mut trim_patterns = Vec::<Box<dyn NBTTag>>::new();
        for trim_pattern in value.trim_patterns {
            trim_patterns.push(trim_pattern.to_nbt())
        }

        let mut trim_materials = Vec::<Box<dyn NBTTag>>::new();
        for trim_material in value.trim_materials {
            trim_materials.push(trim_material.to_nbt())
        }

        let mut chat_types = Vec::<Box<dyn NBTTag>>::new();
        for chat_type in value.chat_types {
            chat_types.push(chat_type.to_nbt())
        }

        let mut dimension_types = Vec::new();
        for dimension_type in value.dimension_types {
            dimension_types.push(dimension_type.to_nbt());
        }

        let mut damage_types = Vec::new();
        for damage_type in value.damage_types {
            damage_types.push(damage_type.to_nbt());
        }

        let mut biomes = Vec::<Box<dyn NBTTag>>::new();
        for biome in value.biomes {
            biomes.push(biome.to_nbt())
        }

        NBT(
            Some("REGISTRY".to_string()),
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
