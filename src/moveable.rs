use bevy::ecs::component::Component;

use crate::characters::Direction;

#[derive(Clone)]
pub(crate) struct Speed(pub f32);

/// Component used to store a moveable entity's current direction
/// and speed.
#[allow(unused)]
#[derive(Component)]
pub(crate) struct Moveable {
    pub(crate) direction: Direction,
    pub(crate) speed: Speed,
}
