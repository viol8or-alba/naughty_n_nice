use bevy::ecs::component::Component;

#[derive(Component, Clone)]
pub(crate) struct Indices{
    pub(crate) foward_start: usize,
    pub(crate) foward_end: usize,
    pub(crate) back_start: usize,
    pub(crate) back_end: usize,
    pub(crate) left_start: usize,
    pub(crate) left_end: usize,
    pub(crate) right_start: usize,
    pub(crate) right_end: usize,
}