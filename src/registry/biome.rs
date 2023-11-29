use mclib::nbt::{IntoNBTTag, NBTTag};

/// Biome special effects.
pub struct BiomeEffects {
    /// The color of the fog effect when looking past the view distance.
    pub fog_color: i32,
    /// The tint color of the water blocks.
    pub water_color: i32,
    /// The color of the fog effect when looking past the view distance when underwater.
    pub water_fog_color: i32,
    /// The color of the sky.
    pub sky_color: i32,
    // TODO optional values
}

impl IntoNBTTag for BiomeEffects {
    fn to_nbt(self) -> Box<dyn NBTTag> {
        vec![
            ("fog_color", self.fog_color.to_nbt()),
            ("water_color", self.water_color.to_nbt()),
            ("water_fog_color", self.water_fog_color.to_nbt()),
            ("sky_color", self.sky_color.to_nbt()),
        ]
        .to_nbt()
    }
}

/// The `minecraft:worldgen/biome` registry.
/// It defines several aesthetic characteristics of the biomes present in the game.
pub struct Biome {
    /// Determines whether or not the biome has precipitation
    has_precipitation: bool,
    /// The temperature factor of the biome.
    /// Affects foliage and grass color if they are not explicitly set.
    temperature: f32,
    /// Modifier that affects the resulting temperature.
    temperature_modifier: Option<f32>,
    /// The downfall factor of the biome.
    /// Affects foliage and grass color if they are not explicitly set.
    downfall: f32,
    /// Biome special effects.
    effects: BiomeEffects,
}

impl IntoNBTTag for Biome {
    fn to_nbt(self) -> Box<dyn NBTTag> {
        let mut result = vec![
            ("has_precipitation", (self.has_precipitation as i8).to_nbt()),
            ("temperature", self.temperature.to_nbt()),
            ("downfall", self.downfall.to_nbt()),
            ("effects", self.effects.to_nbt()),
        ];
        if let Some(temperature_modifier) = self.temperature_modifier {
            result.push(("temperature_modifier", temperature_modifier.to_nbt()));
        }
        result.to_nbt()
    }
}
