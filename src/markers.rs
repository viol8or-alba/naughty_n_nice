use bevy::ecs::component::Component;

#[derive(Component)]
pub(crate) struct CameraMarker;

/// Marker for entities that represent a game character. This can be
/// the player or an NPC or an enemy.
#[derive(Component, Clone)]
pub(crate) struct CharacterMarker;
