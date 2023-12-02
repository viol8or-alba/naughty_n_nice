use bevy::{time::Timer, ecs::component::Component};

#[derive(Component, Clone)]
pub(crate) struct AnimationTimer(pub(crate) Timer);