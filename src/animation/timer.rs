use bevy::{ecs::component::Component, time::Timer};

#[derive(Component, Clone)]
pub(crate) struct AnimationTimer(pub(crate) Timer);
