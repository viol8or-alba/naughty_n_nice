use bevy::ecs::component::Component;

#[derive(Component, Clone)]
pub(crate) struct AnimationIndices {
    pub(crate) forward_start: usize,
    pub(crate) forward_end: usize,
    pub(crate) back_start: usize,
    pub(crate) back_end: usize,
    pub(crate) left_start: usize,
    pub(crate) left_end: usize,
    pub(crate) right_start: usize,
    pub(crate) right_end: usize,
}
