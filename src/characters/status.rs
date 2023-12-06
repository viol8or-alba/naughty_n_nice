use bevy::ecs::component::Component;

use super::CharacterState;

/// Used to store the current [`CharacterState`] of a character.
#[derive(Component)]
pub(crate) struct Status {
    pub(crate) state: CharacterState,
}
