use bevy::{ecs::component::Component, time::Timer};

/// Timer for animation, wraps a [`Timer`] object.
#[derive(Component, Clone)]
pub(crate) struct AnimationTimer(pub(crate) Timer);
