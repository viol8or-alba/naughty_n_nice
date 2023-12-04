use bevy::ecs::component::Component;

use crate::characters::Direction;

#[derive(Clone)]
pub(crate) struct Speed(pub f32);

#[derive(Component, Clone)]
pub(crate) struct Moveable {
    pub(crate) direction: Direction,
    pub(crate) speed: Speed,
}
