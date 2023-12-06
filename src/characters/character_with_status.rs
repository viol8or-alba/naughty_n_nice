use bevy::ecs::bundle::Bundle;

use super::{status::Status, BasicCharacter};

/// This [`Bundle`] adds a [`Status`] to a [`BasicCharacter`].
#[derive(Bundle)]
pub(crate) struct CharacterWithStatus {
    pub(crate) basic_character: BasicCharacter,
    pub(crate) status: Status,
}
