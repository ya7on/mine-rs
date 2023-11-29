use mclib::nbt::{IntoNBTTag, NBTTag};

/// The minecraft:dimension_type registry. It defines the types of dimension that can be attributed to a world, along with all their characteristics.
/// Dimension type entries are referenced in the Login (play) and Respawn packets.
pub struct DimensionType {
    /// If set, the time of the day fixed to the specified value.
    /// Allowed values vary between 0 and 24000.
    pub fixed_time: Option<i64>,
    /// Whether the dimension has skylight access or not.
    pub has_skylight: bool,
    /// Whether the dimension has a bedrock ceiling or not. When true, causes lava to spread faster.
    pub has_ceiling: bool,
    /// Whether the dimensions behaves like the nether (water evaporates and sponges dry) or not.
    /// Also causes lava to spread thinner.
    pub ultrawarm: bool,
    /// When false, compasses spin randomly. When true, nether portals can spawn zombified piglins.
    pub natural: bool,
    /// The multiplier applied to coordinates when traveling to the dimension.
    pub coordinate_scale: f64,
    /// Whether players can use a bed to sleep.
    pub bed_works: bool,
    /// Whether players can charge and use respawn anchors.
    pub respawn_anchor_works: bool,
    /// The minimum Y level.
    /// Allowed values vary between -2032 and 2031, and must also be a multiple of 16.
    ///
    /// min_y + height cannot exceed 2032.
    pub min_y: i32,
    /// The maximum height.
    /// Allowed values vary between 16 and 4064, and must also be a multiple of 16.
    ///
    /// min_y + height cannot exceed 2032.
    pub height: i32,
    /// The maximum height to which chorus fruits and nether portals can bring players within this dimension. (Must be lower than height)
    /// Allowed values vary between 0 and 4064, and must also be a multiple of 16.
    ///
    /// logical_height cannot exceed the height.
    pub logical_height: i32,
    /// A resource location defining what block tag to use for infiniburn.
    ///
    /// "#" or minecraft resource "#minecraft:...".
    pub infiniburn: String,
    /// Defines special dimensional effects, which includes:
    /// * Cloud level: Height at which clouds appear, if at all.
    /// * Sky type: Whether it's the normal sky with sun and moon; the low-visibility, foggy sky of the nether; or the static sky of the end.
    /// * Forced light map: Whether a bright light map is forced, siimilar to the night vision effect.
    /// * Constant ambient light: Whether blocks have shade on their faces.
    ///
    /// Can be either:
    /// * `minecraft:overworld`, for clouds at 192, normal sky type, normal light map and normal ambient light.
    /// * `minecraft:the_nether`, for no clouds, nether sky type, normal light map and constant ambient light.
    /// * `minecraft:the_end`, for no clouds, end sky type, forced light map and normal ambient light.
    pub effects: String,
    /// How much light the dimension has. Used as interpolation factor when calculating the brightness generated from sky light.
    /// The default values are 0.0 and 0.1, 0.1 for the nether and 0.0 for the other dimensions.
    pub ambient_light: f32,
    /// Whether piglins shake and transform to zombified piglins.
    pub piglin_safe: bool,
    /// Whether players with the Bad Omen effect can cause a raid.
    pub has_raids: bool,
    /// During a monster spawn attempt, this is the maximum allowed light level for it to succeed.
    /// It can be either a fixed value, or one of several types of distributions.
    pub monster_spawn_light_level: i32, // TODO
    /// Maximum allowed block light level monster spawn attempts to happen.
    ///
    /// Allowed values vary between 0 and 15.
    /// The default values are 0 and 15, 15 for the nether (where monsters can spawn anywhere)
    /// and 0 for other dimensions (where monsters can only spawn naturally in complete darkness).
    pub monster_spawn_block_light_limit: i32,
}

impl IntoNBTTag for DimensionType {
    fn to_nbt(self) -> Box<dyn NBTTag> {
        let mut result = vec![
            ("has_skylight", (self.has_skylight as i8).to_nbt()),
            ("has_ceiling", (self.has_ceiling as i8).to_nbt()),
            ("ultrawarm", (self.ultrawarm as i8).to_nbt()),
            ("natural", (self.natural as i8).to_nbt()),
            ("coordinate_scale", self.coordinate_scale.to_nbt()),
            ("bed_works", (self.bed_works as i8).to_nbt()),
            (
                "respawn_anchor_works",
                (self.respawn_anchor_works as i8).to_nbt(),
            ),
            ("min_y", self.min_y.to_nbt()),
            ("height", self.height.to_nbt()),
            ("logical_height", self.logical_height.to_nbt()),
            ("infiniburn", self.infiniburn.to_nbt()),
            ("effects", self.effects.to_nbt()),
            ("ambient_light", self.ambient_light.to_nbt()),
            ("piglin_safe", (self.piglin_safe as i8).to_nbt()),
            ("has_raids", (self.has_raids as i8).to_nbt()),
            (
                "monster_spawn_light_level",
                self.monster_spawn_light_level.to_nbt(),
            ),
            (
                "monster_spawn_block_light_limit",
                self.monster_spawn_block_light_limit.to_nbt(),
            ),
        ];

        if let Some(fixed_time) = self.fixed_time {
            result.push(("fixed_time", fixed_time.to_nbt()));
        }

        result.to_nbt()
    }
}
