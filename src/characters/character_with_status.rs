use bevy::ecs::bundle::Bundle;

use super::{status::Status, BasicCharacter};

#[derive(Bundle)]
pub(crate) struct CharacterWithStatus {
    pub(crate) basic_character: BasicCharacter,
    pub(crate) status: Status,
}
