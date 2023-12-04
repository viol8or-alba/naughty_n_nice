use bevy::ecs::component::Component;

#[derive(Component)]
pub(crate) struct CameraMarker;

#[derive(Component, Clone)]
pub(crate) struct CharacterMarker;
