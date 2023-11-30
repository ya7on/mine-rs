use mclib::nbt::{IntoNBTTag, NBTTag};
use serde::Deserialize;

/// The Registry Entry Compound Tag specifies a single entry of a Registry.
#[derive(Deserialize, Debug)]
pub struct RegistryEntry<T: IntoNBTTag> {
    name: String,
    id: i32,
    element: T,
}

impl<T: IntoNBTTag> IntoNBTTag for RegistryEntry<T> {
    fn to_nbt(self) -> Box<dyn NBTTag> {
        vec![
            ("name", self.name.to_nbt()),
            ("id", self.id.to_nbt()),
            ("element", self.element.to_nbt()),
        ]
        .to_nbt()
    }
}
