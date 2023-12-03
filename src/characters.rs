mod basic_character;

pub(crate) use basic_character::BasicCharacter;
use bevy::ecs::component::Component;

#[derive(Component, Clone, Copy)]
pub(crate) enum Direction {
    Static,
    Back,
    Forward,
    Left,
    Right,
}
