use bevy::ecs::bundle::Bundle;

use super::{status::Status, BasicCharacter, Inventory};

/// This [`Bundle`] adds a [`Status`] and an [`Inventory`]to a [`BasicCharacter`].
#[derive(Bundle)]
pub(crate) struct CharacterWithStatus {
    pub(crate) basic_character: BasicCharacter,
    pub(crate) status: Status,
    pub(crate) inventory: Inventory,
}
