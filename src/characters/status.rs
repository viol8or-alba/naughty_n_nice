use bevy::ecs::component::Component;

use super::CharacterState;

#[derive(Component)]
pub(crate) struct Status {
    pub(crate) state: CharacterState,
}
